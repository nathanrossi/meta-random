use super::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use nix::sys::signal::{Signal, SigSet, SigmaskHow};
use nix::sys::signalfd::{SignalFd, SfdFlags};
use nix::unistd::Pid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ServiceState
{
	Unknown,
	Inactive,
	Starting,
	Running,
	Completed,
	Error,
}

pub trait Service
{
	fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext);
	fn state(&self) -> ServiceState;
	fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext);
	fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext);

	fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : Pid, status : std::process::ExitStatus);
	fn device_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, event : &uevent::EventData);
}

pub struct ServiceContext
{
	processes : Vec<Pid>,
	uevent_subsystems : Vec<String>,
	fds : Vec<RawFd>,
}

impl ServiceContext
{
	pub fn new() -> Self
	{
		return Self {
				processes : Vec::new(),
				uevent_subsystems : Vec::new(),
				fds : Vec::new(),
			};
	}

	pub fn register_child(&mut self, child : &std::process::Child)
	{
		self.processes.push(Pid::from_raw(child.id() as i32));
	}

	pub fn register_device_subsystem(&mut self, subsystem : &str)
	{
		self.uevent_subsystems.push(subsystem.to_owned());
	}
}

pub struct ServiceInstance<'a>
{
	service : Box<dyn Service + 'a>,
	context : ServiceContext,
}

impl<'a> ServiceInstance<'a>
{
	pub fn new(service : impl Service + 'a) -> Self
	{
		return Self {
				service : Box::new(service),
				context : ServiceContext::new(),
			};
	}

	pub fn setup(&mut self, runtime : &Runtime)
	{
		self.service.setup(runtime, &mut self.context);
	}

	pub fn state(&self) -> ServiceState
	{
		return self.service.state();
	}

	pub fn start(&mut self, runtime : &Runtime)
	{
		self.service.start(runtime, &mut self.context);
	}

	pub fn stop(&mut self, runtime : &Runtime)
	{
		self.service.stop(runtime, &mut self.context);
	}

	pub fn process_event(&mut self, runtime : &Runtime, pid : Pid, status : std::process::ExitStatus) -> bool
	{
		for p in self.context.processes.iter() {
			if pid == *p {
				self.service.process_event(runtime, &mut self.context, pid, status);
				return true;
			}
		}
		return false;
	}

	pub fn device_event(&mut self, runtime : &Runtime, event : &uevent::EventData) -> bool
	{
		if let Some(subsystem) = event.properties.get("SUBSYSTEM") {
			for name in self.context.uevent_subsystems.iter() {
				if name == subsystem {
					self.service.device_event(runtime, &mut self.context, event);
					return true;
				}
			}
		}
		return false;
	}
}

pub struct ServiceManager<'a>
{
	services : Vec<Rc<RefCell<ServiceInstance<'a>>>>,
}

impl<'a> ServiceManager<'a>
{
	pub fn new() -> Self
	{
		return Self { services : Vec::new() };
	}

	pub fn add_service(&mut self, runtime : &Runtime, service : impl Service + 'a, enabled : bool) -> Rc<RefCell<ServiceInstance<'a>>>
	{
		let instance = Rc::new(RefCell::new(ServiceInstance::new(service)));
		instance.borrow_mut().setup(runtime);
		if enabled {
			instance.borrow_mut().start(runtime);
		}
		self.services.push(instance.clone());
		return instance;
	}

	pub fn process_event(&mut self, runtime : &Runtime, pid : Pid, status : std::process::ExitStatus) -> bool
	{
		for service in self.services.iter_mut() {
			if service.borrow_mut().process_event(runtime, pid, status) {
				return true;
			}
		}
		return false;
	}

	pub fn device_event(&mut self, runtime : &Runtime, event : &uevent::EventData) -> bool
	{
		let mut handled = false;
		for service in self.services.iter_mut() {
			if service.borrow_mut().device_event(runtime, event) {
				handled = true;
			}
		}
		return handled;
	}
}

pub struct Runtime
{
	poll : mio::Poll,
	signalfd : SignalFd,
	uevent : uevent::Socket,
	fds : Vec<(usize, RawFd)>,
}

impl Runtime
{
	pub fn new() -> Result<Self>
	{
		let poll = mio::Poll::new()?;

		let mut sigset = SigSet::empty();
		sigset.add(Signal::SIGCHLD);
		nix::sys::signal::sigprocmask(SigmaskHow::SIG_BLOCK, Some(&sigset), None)?;
		let sigfd = SignalFd::with_flags(&sigset, SfdFlags::SFD_NONBLOCK | SfdFlags::SFD_CLOEXEC)?;
		poll.register(&mut mio::unix::EventedFd(&sigfd.as_raw_fd()), mio::Token(0), mio::Ready::readable(), mio::PollOpt::edge())?;

		let socket = uevent::Socket::open_blocking(false)?;
		poll.register(&mut mio::unix::EventedFd(&socket.as_raw_fd()), mio::Token(1), mio::Ready::readable(), mio::PollOpt::edge())?;

		return Ok(Runtime {
				poll : poll,
				signalfd : sigfd,
				uevent : socket,
				// ueventcallbacks : Vec::new(),
				fds : Vec::new(),
			});
	}

	fn get_unused_token(&self) -> usize
	{
		// return first available token
		if self.fds.len() == 0 {
			return 1;
		}

		let mut last = 10;
		for (token, _) in self.fds.iter() {
			if (token - last) > 1 {
				return last + 1
			}
			last = *token;
		}
		return last + 1;
	}

	pub fn register(&mut self, fd : RawFd, read : bool, write : bool) -> Result<()>
	{
		let token = self.get_unused_token();
		self.fds.push((token, fd));

		let mut ready = mio::Ready::empty();
		if read {
			ready.insert(mio::Ready::readable());
		}
		if write {
			ready.insert(mio::Ready::writable());
		}
		self.poll.register(&mut mio::unix::EventedFd(&fd), mio::Token(token), ready, mio::PollOpt::edge())?;
		return Ok(());
	}

	pub fn poll(&mut self, manager : &mut ServiceManager) -> Result<()>
	{
		let mut events = mio::Events::with_capacity(64);
		loop {
			self.poll.poll(&mut events, None)?;
			for event in &events {
				if let Err(e) = self.handle_event(manager, event) {
					println!("error!: {:?}", e);
				}
			}
		}
	}

	pub fn poll_once(&mut self, manager : &mut ServiceManager) -> Result<()>
	{
		let mut events = mio::Events::with_capacity(64);
		loop {
			self.poll.poll(&mut events, None)?;
			let mut handled = 0;
			for event in &events {
				if self.handle_event(manager, event)? {
					handled += 1;
				}
			}
			if handled != 0 {
				return Ok(());
			}
		}
	}

	pub fn poll_service_ready<'a>(&mut self, manager : &mut ServiceManager, service : &Rc<RefCell<ServiceInstance<'a>>>) -> Result<()>
	{
		loop {
			match service.borrow().state() {
				ServiceState::Completed => { return Ok(()); }
				ServiceState::Running => { return Ok(()); }
				_ => {}
			}

			self.poll_once(manager)?;
		}
	}

	fn handle_event(&mut self, manager : &mut ServiceManager, event : mio::Event) -> Result<bool>
	{
		if event.token() == mio::Token(0) {
			// signal
			if let Some(_) = self.signalfd.read_signal()? {
				let mut handled = false;
				// loop through any waiting child processes
				loop {
					if self.check_processes(manager)? {
						handled = true;
					} else {
						break;
					}
				}
				return Ok(handled);
			}
		} else if event.token() == mio::Token(1) {
			let message = self.uevent.read()?;
			if let Some(eventdata) = message {
				// println!("[uevent] got event");
				// for (k, v) in eventdata.properties.iter() {
					// println!("    {} = {}", k, v);
				// }

				if manager.device_event(self, &eventdata) {
					return Ok(true);
				}
			}
		} else if event.token() >= mio::Token(10) {
			for (token, fd) in self.fds.iter() {
				if event.token() == mio::Token(*token) {
					// callback(&self, *fd);
					return Ok(true);
				}
			}
		}
		return Ok(false);
	}

	fn check_processes(&mut self, manager : &mut ServiceManager) -> Result<bool>
	{
		match nix::sys::wait::waitpid(None, Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
			Err(e) => {
					if let nix::Error::Sys(errno) = e {
						if errno == nix::errno::Errno::ECHILD {
							return Ok(false); // no children to wait for
						}
					}
					return Err(Box::new(e));
				}
			Ok(status) => {
					match status {
						nix::sys::wait::WaitStatus::StillAlive => {
								return Ok(false);
							}
						nix::sys::wait::WaitStatus::Exited(pid, status) => {
								let estatus = std::os::unix::process::ExitStatusExt::from_raw(status);
								if manager.process_event(self, pid, estatus) {
									return Ok(true);
								}

								// TODO: handle orphan
								println!("reaped orphan (pid = {}, status = {})", pid, status);

								return Ok(false);
							}
						nix::sys::wait::WaitStatus::Signaled(pid, signal, _) => {
								let estatus = std::os::unix::process::ExitStatusExt::from_raw(0); // TODO: convert signal to exitstatus
								if manager.process_event(self, pid, estatus) {
									return Ok(true);
								}

								// TODO: handle orphan
								println!("reaped orphan (pid = {}, signal = {})", pid, signal);

								return Ok(false);
							}
						_ => {
							println!("check_processes: ???? {:?}", status);
							return Ok(false);
						}
					}
				}
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use std::os::unix::io::AsRawFd;

	#[test]
	fn runtime_process()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new().unwrap();
		let flag = std::rc::Rc::new(std::sync::atomic::AtomicUsize::new(0));

		struct TestService
		{
			flag : std::rc::Rc<std::sync::atomic::AtomicUsize>,
			child : Option<std::process::Child>,
		}

		impl Service for TestService
		{
			fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext)
			{
				println!("setup child");
				let child = std::process::Command::new("echo").spawn().unwrap();
				println!("spawn child");
				context.register_child(&child);
				self.child = Some(child);
			}

			fn state(&self) -> ServiceState { return ServiceState::Unknown; }
			fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext) {}
			fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext) {}

			fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : Pid, status : std::process::ExitStatus)
			{
				if let Some(child) = &self.child {
					if pid == Pid::from_raw(child.id() as i32) {
						println!("correct child completed");
						self.flag.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
					}
				}
			}

			fn device_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, event : &uevent::EventData) {}
		}

		let service = TestService { flag : flag.clone(), child : None };
		println!("starting service");

		manager.add_service(&rt, service, false);
		rt.poll_once(&mut manager).unwrap();

		assert_eq!(flag.load(std::sync::atomic::Ordering::Relaxed), 1);
	}

	/*
	#[test]
	fn runtime_fd()
	{
		let flag = std::sync::atomic::AtomicUsize::new(0);
		let mut rt = Runtime::new().unwrap();

		let (pipeout, pipein) = nix::unistd::pipe2(nix::fcntl::OFlag::O_NONBLOCK).unwrap();

		rt.register(pipeout, true, false, |_, _| {
				flag.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
			}).unwrap();

		nix::unistd::write(pipein, &[0x0, 0x0]).unwrap();

		rt.poll_once().unwrap();
		assert_eq!(flag.load(std::sync::atomic::Ordering::Relaxed), 1);
	}
	*/

	#[test]
	fn runtime_device_event()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new().unwrap();
		let flag = std::rc::Rc::new(std::sync::atomic::AtomicUsize::new(0));

		struct TestService
		{
			flag : std::rc::Rc<std::sync::atomic::AtomicUsize>,
		}

		impl Service for TestService
		{
			fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext)
			{
				println!("setup device event");
				context.register_device_subsystem("tty");
			}

			fn state(&self) -> ServiceState { return ServiceState::Unknown; }
			fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext) {}
			fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext) {}

			fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : Pid, status : std::process::ExitStatus) {}

			fn device_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, event : &uevent::EventData)
			{
				println!("got uevent data");
				self.flag.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
			}
		}

		let service = TestService { flag : flag.clone() };
		println!("starting service");

		manager.add_service(&rt, service, false);

		// TODO: Cannot generate mock uevent data currently.
		// rt.poll_once(&mut manager).unwrap();

		// assert_eq!(flag.load(std::sync::atomic::Ordering::Relaxed), 1);
	}

	#[test]
	fn runtime_service_poll_ready()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new().unwrap();
		let flag = std::rc::Rc::new(std::sync::atomic::AtomicUsize::new(0));

		struct TestService
		{
			flag : std::rc::Rc<std::sync::atomic::AtomicUsize>,
			child : Option<std::process::Child>,
		}

		impl Service for TestService
		{
			fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext)
			{
				println!("setup child");
				let child = std::process::Command::new("echo").spawn().unwrap();
				println!("spawn child");
				context.register_child(&child);
				self.child = Some(child);
			}

			fn state(&self) -> ServiceState
			{
				if let Some(_) = self.child {
					if self.flag.load(std::sync::atomic::Ordering::Relaxed) >= 1 {
						return ServiceState::Completed;
					}
					return ServiceState::Starting;
				}
				return ServiceState::Inactive;
			}
			fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext) {}
			fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext) {}

			fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : Pid, status : std::process::ExitStatus)
			{
				if let Some(child) = &self.child {
					if pid == Pid::from_raw(child.id() as i32) {
						println!("correct child completed");
						self.flag.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
					}
				}
			}

			fn device_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, event : &uevent::EventData) {}
		}

		let service = TestService { flag : flag.clone(), child : None };
		println!("starting service");

		let service = manager.add_service(&rt, service, false);

		println!("waiting for service");
		rt.poll_service_ready(&mut manager, &service).unwrap();
	}
}

