use super::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use nix::sys::signal::{Signal, SigSet, SigmaskHow};
use nix::sys::signalfd::{SignalFd, SfdFlags};
use service::{ServiceRef, ServiceManager, ServiceState};
use logging::Logger;
use procfs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Runtime<'a>
{
	pub logger : Logger<'a>,
	poll : mio::Poll,
	signalfd : SignalFd,
	uevent : uevent::Socket,
	fds : Vec<(usize, RawFd)>,
}

#[allow(dead_code)]
impl<'a> Runtime<'a>
{
	pub fn new(logger : Logger<'a>) -> Result<Self>
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
				logger : logger,
				poll : poll,
				signalfd : sigfd,
				uevent : socket,
				fds : Vec::new(),
			});
	}

	pub fn new_default_logger() -> Result<Self>
	{
		let mut logger = Logger::new();
		logger.add(std::io::stdout());
		return Runtime::new(logger);
	}

	pub fn new_test_logger(name : &str) -> Result<Self>
	{
		let mut logger = Logger::new();
		logger.prefix = Some(name.to_owned());
		logger.add(std::io::stdout());
		return Runtime::new(logger);
	}

	fn get_unused_token(&mut self) -> usize
	{
		let mut last : usize = 1;
		for (token, _) in self.fds.iter() {
			if *token > last {
				last = *token;
			}
		}
		return last + 1;
	}

	fn get_fd_from_token(&self, token : usize) -> Option<RawFd>
	{
		for (etoken, efd) in self.fds.iter() {
			if *etoken == token {
				return Some(*efd);
			}
		}
		return None;
	}

	pub fn register<F: AsRawFd>(&mut self, fd : &F, read : bool, write : bool) -> Result<()>
	{
		let rfd = fd.as_raw_fd();
		let token = self.get_unused_token();
		self.fds.push((token, rfd));

		let mut ready = mio::Ready::empty();
		if read {
			ready.insert(mio::Ready::readable());
		}
		if write {
			ready.insert(mio::Ready::writable());
		}
		self.poll.register(&mut mio::unix::EventedFd(&rfd), mio::Token(token), ready, mio::PollOpt::edge())?;
		return Ok(());
	}

	pub fn poll(&mut self, manager : &mut ServiceManager, once : bool) -> Result<()>
	{
		let mut events = mio::Events::with_capacity(64);
		loop {
			self.poll.poll(&mut events, None)?;
			let mut washandled = false;
			for event in &events {
				match self.handle_event(manager, event) {
					Ok(handled) => {
							if handled {
								washandled = true;
							} else {
								break;
							}
						},
					Err(e) => {
							self.logger.service_log("runtime", &format!("error when polling: {:?}", e));
							continue;
						}
				}
			}

			if once && washandled {
				return Ok(());
			}
		}
	}

	// TODO: replace this
	pub fn poll_once(&mut self, manager : &mut ServiceManager) -> Result<()>
	{
		return self.poll(manager, true);
	}

	pub fn poll_service_ready<'s>(&mut self, manager : &mut ServiceManager, service : &ServiceRef<'s>) -> Result<()>
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
		let tokenid = usize::from(event.token());
		if tokenid == 0 {
			let mut handled = false;
			loop {
				// signal
				let signal = self.signalfd.read_signal()?;
				if let Some(_) = signal {
					// loop through any waiting child processes
					loop {
						if self.check_processes(manager)? {
							handled = true;
						} else {
							break;
						}
					}
				} else {
					return Ok(handled);
				}
			}
		} else if tokenid == 1 {
			let mut handled = true;
			loop {
				if let Some(eventdata) = self.uevent.read()? {
					if manager.device_event(self, &eventdata) {
						handled = true;
					}
				} else {
					return Ok(handled);
				}
			}
		} else if let Some(fd) = self.get_fd_from_token(tokenid) {
			return Ok(manager.fd_event(self, fd));
		}
		return Ok(false);
	}

	fn check_processes(&mut self, manager : &mut ServiceManager) -> Result<bool>
	{
		let flags =
			nix::sys::wait::WaitPidFlag::WEXITED |
			nix::sys::wait::WaitPidFlag::WNOHANG |
			nix::sys::wait::WaitPidFlag::WNOWAIT;
		match nix::sys::wait::waitid(nix::sys::wait::Id::All, flags) {
			Err(e) => {
				if let nix::Error::ECHILD = e {
					return Ok(false); // no children to wait for
				} else {
					self.logger.service_log("runtime", &format!("waitpid error ({})", e));
				}
				return Err(Box::new(e));
			}
			Ok(status) => {
				match status {
					nix::sys::wait::WaitStatus::Exited(pid, _) => {
						return self.handle_process(manager, pid);
					}
					nix::sys::wait::WaitStatus::Signaled(pid, _, _) => {
						return self.handle_process(manager, pid);
					}
					_ => {
						return Ok(false);
					}
				}
			}
		}
	}

	fn handle_process(&mut self, manager : &mut ServiceManager, pid : nix::unistd::Pid) -> Result<bool>
	{
		let flags = nix::sys::wait::WaitPidFlag::WEXITED | nix::sys::wait::WaitPidFlag::WNOHANG;
		let comm = procfs::process_comm(pid).unwrap_or_default();

		match nix::sys::wait::waitid(nix::sys::wait::Id::Pid(pid), flags) {
			Err(e) => {
				if let nix::Error::ECHILD = e {
					return Ok(false); // no children to wait for
				} else {
					self.logger.service_log("runtime", &format!("waitpid error ({})", e));
				}
				return Err(Box::new(e));
			},
			Ok(status) => {
				match status {
					nix::sys::wait::WaitStatus::StillAlive => {
						return Ok(false);
					}
					nix::sys::wait::WaitStatus::Exited(pid, status) => {
						let estatus = std::os::unix::process::ExitStatusExt::from_raw(status);
						// self.logger.service_log("runtime", &format!("exited process '{}' (pid = {}, status = {})", comm, pid, status));

						if manager.process_event(self, pid, estatus) {
							return Ok(true);
						}

						// TODO: handle orphan
						self.logger.service_log("runtime", &format!("reaped orphan '{}' (pid = {}, status = {})", comm, pid, status));

						return Ok(false);
					}
					nix::sys::wait::WaitStatus::Signaled(pid, signal, _) => {
						let estatus = std::os::unix::process::ExitStatusExt::from_raw(0); // TODO: convert signal to exitstatus
						// self.logger.service_log("runtime", &format!("signaled process '{}' (pid = {}, status = {})", comm, pid, signal));

						if manager.process_event(self, pid, estatus) {
							return Ok(true);
						}

						// TODO: handle orphan
						self.logger.service_log("runtime", &format!("reaped orphan '{}' (pid = {}, signal = {})", comm, pid, signal));

						return Ok(false);
					}
					_ => {
						self.logger.service_log("runtime", &format!("check_processes: ??? {:?}", status));
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
	use super::super::service::{Service, ServiceEvent};

	macro_rules! test_name {
		() => {{
			fn f() {}
			fn type_name_of<T>(_: T) -> &'static str {
				std::any::type_name::<T>()
			}
			let name = type_name_of(f);
			&name[..name.len() - 3]
		}}
	}

	struct TestProcessService
	{
		flag : std::rc::Rc<std::sync::atomic::AtomicUsize>,
		child : Option<std::process::Child>,
	}

	impl Service for TestProcessService
	{
		fn setup(&mut self, runtime : &mut Runtime)
		{
			runtime.logger.log("setup child");
			let child = std::process::Command::new("echo")
				.stdout(std::process::Stdio::null())
				.stderr(std::process::Stdio::null())
				.spawn().unwrap();
			runtime.logger.log("spawn child");
			runtime.logger.log(&format!("pid = {}", child.id()));
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
		fn start(&mut self, _runtime : &mut Runtime) {}
		fn stop(&mut self, _runtime : &mut Runtime) {}

		fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
		{
			if let ServiceEvent::ProcessExited(pid, _) = event {
				if let Some(child) = &self.child {
					if child.id() == pid {
						runtime.logger.log("correct child completed");
						self.flag.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
						return true;
					}
				}
			}
			return false;
		}
	}

	// NOTE: The following tests rely on signalfd and sigprocmask such that the SIGCHLD need to be
	// delivered to the currently running process. Because of this running the tests in multiple
	// threads is problematic.

	#[test]
	fn runtime_process()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new_test_logger(test_name!()).unwrap();
		let flag = std::rc::Rc::new(std::sync::atomic::AtomicUsize::new(0));

		let service = TestProcessService { flag : flag.clone(), child : None };
		rt.logger.log("starting service");

		manager.add_service(&mut rt, service, false);
		rt.poll_once(&mut manager).unwrap();

		assert_eq!(flag.load(std::sync::atomic::Ordering::Relaxed), 1);
	}

	#[test]
	fn runtime_service_poll_ready()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new_test_logger(test_name!()).unwrap();
		let flag = std::rc::Rc::new(std::sync::atomic::AtomicUsize::new(0));

		let service = TestProcessService { flag : flag.clone(), child : None };
		rt.logger.log("starting service");

		let service = manager.add_service(&mut rt, service, false);

		rt.logger.log("waiting for service");
		rt.poll_service_ready(&mut manager, &service).unwrap();
	}

	#[test]
	fn runtime_fd()
	{
		let flag = std::rc::Rc::new(std::sync::atomic::AtomicUsize::new(0));
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new_test_logger(test_name!()).unwrap();

		struct TestService
		{
			flag : std::rc::Rc<std::sync::atomic::AtomicUsize>,
			fd : RawFd,
		}

		impl Service for TestService
		{
			fn setup(&mut self, runtime : &mut Runtime)
			{
				runtime.register(&self.fd, true, false).unwrap();
			}

			fn state(&self) -> ServiceState { return ServiceState::Unknown; }
			fn start(&mut self, _runtime : &mut Runtime) {}
			fn stop(&mut self, _runtime : &mut Runtime) {}

			fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
			{
				runtime.logger.log(&format!("got event, {:?}", event));
				if let ServiceEvent::Fd(efd) = event {
					runtime.logger.log(&format!("got fd event, fd = {}", efd));
					if efd == self.fd {
						runtime.logger.log("correct fd event");
						self.flag.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
						return true;
					}
				}
				return false;
			}
		}

		rt.logger.log("setup pipe");
		let (pipeout, pipein) = nix::unistd::pipe2(nix::fcntl::OFlag::O_NONBLOCK).unwrap();

		let service = TestService { flag : flag.clone(), fd : pipeout };
		rt.logger.log("starting service");
		manager.add_service(&mut rt, service, false);

		rt.logger.log("send data to pipe");
		nix::unistd::write(pipein, &[0x0, 0x0]).unwrap();

		rt.logger.log("poll_once");
		rt.poll_once(&mut manager).unwrap();
		assert_eq!(flag.load(std::sync::atomic::Ordering::Relaxed), 1);
	}

	#[test]
	fn runtime_device_event()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new_test_logger(test_name!()).unwrap();
		let flag = std::rc::Rc::new(std::sync::atomic::AtomicUsize::new(0));

		struct TestService
		{
			flag : std::rc::Rc<std::sync::atomic::AtomicUsize>,
		}

		impl Service for TestService
		{
			fn setup(&mut self, runtime : &mut Runtime)
			{
				runtime.logger.log("setup device event");
			}

			fn state(&self) -> ServiceState { return ServiceState::Unknown; }
			fn start(&mut self, _runtime : &mut Runtime) {}
			fn stop(&mut self, _runtime : &mut Runtime) {}

			fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
			{
				if let ServiceEvent::Device(_) = event {
					runtime.logger.log("got uevent data");
					self.flag.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
					return true;
				}
				return false;
			}
		}

		let service = TestService { flag : flag.clone() };
		rt.logger.log("starting service");

		manager.add_service(&mut rt, service, false);

		// TODO: Cannot generate mock uevent data currently.
		// rt.poll_once(&mut manager).unwrap();

		// assert_eq!(flag.load(std::sync::atomic::Ordering::Relaxed), 1);
	}
}

