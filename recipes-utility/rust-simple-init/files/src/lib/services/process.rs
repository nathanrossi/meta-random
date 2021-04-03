use std::collections::vec_deque::VecDeque;
use std::process::{Command, Child};
use std::os::unix::io::{RawFd, AsRawFd};
use super::super::*;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct PipeBuffer
{
	pub fd : RawFd,
	lines : VecDeque<Vec<u8>>,
	buf : Vec<u8>,
}

impl PipeBuffer
{
	pub fn new(fd : RawFd) -> Result<Self>
	{
		nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_SETFL(nix::fcntl::OFlag::O_NONBLOCK))?;
		return Ok(Self { fd : fd, lines : VecDeque::new(), buf : Vec::new() });
	}

	pub fn poll(&mut self) -> Option<Vec<u8>>
	{
		if self.lines.len() != 0 {
			return self.lines.pop_front();
		}

		let mut buf : [u8; 1024] = [0; 1024];
		match nix::unistd::read(self.fd, &mut buf) {
			Ok(count) => {
				if count != 0 {
					self.push(&buf[0 .. count]);
					return self.lines.pop_front();
				}
			},
			Err(_) => {},
		}
		return None;
	}

	fn push(&mut self, data : &[u8])
	{
		// look for \n in data, and split it into lines
		for v in data.iter() {
			if *v == b'\n' {
				self.lines.push_back(self.buf.split_off(0));
			} else {
				self.buf.push(*v);
			}
		}
	}
}

enum ProcessState
{
	Stopped,
	Starting,
	Running(Child, PipeBuffer, PipeBuffer),
	Completed,
	Error,
}

pub struct ProcessService
{
	command : Command,
	state : ProcessState,
	oneshot : bool,
	devices : Vec<String>,
}

impl ProcessService
{
	pub fn new(executable : &str, args : &[&str]) -> Self
	{
		let mut command = Command::new(executable);
		command.args(args);
		return Self { command : command, state : ProcessState::Stopped, oneshot : false, devices : Vec::new() };
	}

	pub fn oneshot(executable : &str, args : &[&str]) -> Self
	{
		let mut command = Command::new(executable);
		command.args(args);
		return Self { command : command, state : ProcessState::Stopped, oneshot : true, devices : Vec::new() };
	}

	// TODO: make a "device rule" that can be stored and tested
	pub fn add_device_dependency(&mut self, path : &str) -> &Self
	{
		self.devices.push(path.to_owned());
		return self;
	}

	fn check_devices(&self, runtime : &mut Runtime) -> bool
	{
		for i in &self.devices {
			let path = std::path::Path::new(&i);
			if !path.exists() {
				runtime.logger.service_log("process", &format!("waiting for path {:?}", path));
				return false;
			}
		}
		return true;
	}

	fn check_state(&mut self, runtime : &mut Runtime)
	{
		// Do not attempt to check/change state for inactive states
		match self.state {
			ProcessState::Starting => (),
			_ => return,
		}

		if !self.check_devices(runtime) {
			return;
		}

		if let Ok(child) = self.command
				.stdin(std::process::Stdio::null())
				.stdout(std::process::Stdio::piped())
				.stderr(std::process::Stdio::piped())
				.spawn() {
			runtime.logger.service_log("process", &format!("starting {:?}", self.command));
			if child.stdout.is_none() || child.stderr.is_none() {
				runtime.logger.service_log("process", "failed to register process stdout fd");
				self.state = ProcessState::Error;
			}

			let stdout;
			if let Some(pipe) = &child.stdout {
				if let Ok(pb) = PipeBuffer::new(pipe.as_raw_fd()) {
					if !runtime.register(&pb.fd, true, false).is_ok() {
						runtime.logger.service_log("process", "failed to register process stdout fd");
						self.state = ProcessState::Error;
						return;
					}
					stdout = pb;
				} else {
					runtime.logger.service_log("process", "failed to configure stdout fd");
					self.state = ProcessState::Error;
					return;
				}
			} else {
				self.state = ProcessState::Error;
				return;
			}

			let stderr;
			if let Some(pipe) = &child.stderr {
				if let Ok(pb) = PipeBuffer::new(pipe.as_raw_fd()) {
					if !runtime.register(&pb.fd, true, false).is_ok() {
						runtime.logger.service_log("process", "failed to register process stderr fd");
						self.state = ProcessState::Error;
						return;
					}
					stderr = pb;
				} else {
					runtime.logger.service_log("process", "failed to configure stderr fd");
					self.state = ProcessState::Error;
					return;
				}
			} else {
				self.state = ProcessState::Error;
				return;
			}

			self.state = ProcessState::Running(child, stdout, stderr);
		}
	}
}

impl Service for ProcessService
{
	fn setup(&mut self, _runtime : &mut Runtime) {}

	fn state(&self) -> ServiceState
	{
		match self.state {
			ProcessState::Starting => { return ServiceState::Starting; }
			ProcessState::Running(_, _, _) => {
					if self.oneshot {
						return ServiceState::Starting;
					}
					return ServiceState::Running;
				}
			ProcessState::Completed => { return ServiceState::Completed; }
			ProcessState::Error => { return ServiceState::Error; }
			_ => { return ServiceState::Inactive; }
		}
	}

	fn start(&mut self, runtime : &mut Runtime)
	{
		self.state = ProcessState::Starting;
		self.check_state(runtime);
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
		if let ProcessState::Running(child, _, _) = &mut self.state {
			if let Ok(_) = child.kill() {
				if let Ok(_) = child.wait() {
					self.state = ProcessState::Stopped;
				}
			}
		}
	}

	fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
	{
		match event {
			ServiceEvent::ProcessExited(pid, status) => {
				if let ProcessState::Running(child, stdout, stderr) = &mut self.state {
					if child.id() == pid {
						if !status.success() {
							runtime.logger.service_log("process", &format!("failed to start, exit code = {:?}", status));
							self.state = ProcessState::Error;
							return true;
						}
						// poll stdout/stderr first
						loop {
							if let Some(line) = stdout.poll() {
								if let Ok(s) = String::from_utf8(line) {
									runtime.logger.service_log("process", &format!("stdout: {}", &s));
								}
							} else {
								break;
							}
						}
						loop {
							if let Some(line) = stderr.poll() {
								if let Ok(s) = String::from_utf8(line) {
									runtime.logger.service_log("process", &format!("stderr: {}", &s));
								}
							} else {
								break;
							}
						}

						if self.oneshot {
							self.state = ProcessState::Completed;
						} else {
							self.state = ProcessState::Stopped;
						}
						return true;
					}
				}
			}
			ServiceEvent::Device(_) => {
				self.check_state(runtime);
				return true;
			}
			ServiceEvent::Fd(fd) => {
				if let ProcessState::Running(_, stdout, stderr) = &mut self.state {
					if stdout.fd == fd {
						loop {
							if let Some(line) = stdout.poll() {
								if let Ok(s) = String::from_utf8(line) {
									runtime.logger.service_log("process", &format!("stdout: {}", &s));
								}
							} else {
								break;
							}
						}
						return true;
					}
					if stderr.fd == fd {
						loop {
							if let Some(line) = stderr.poll() {
								if let Ok(s) = String::from_utf8(line) {
									runtime.logger.service_log("process", &format!("stderr: {}", &s));
								}
							} else {
								break;
							}
						}
						return true;
					}
				}
			}
			_ => {}
		}
		return false;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use service::ServiceManager;

	#[test]
	fn process()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new_default_logger().unwrap();
		let service = ProcessService::new("cat", &[]);

		let instance = manager.add_service(&mut rt, service, true);
		rt.poll_service_ready(&mut manager, &instance).unwrap(); // wait for service to start
		assert_eq!(instance.borrow().state(), ServiceState::Running);

		instance.borrow_mut().stop(&mut rt);
		assert_eq!(instance.borrow().state(), ServiceState::Inactive);
	}

	#[test]
	fn process_pipe()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new_default_logger().unwrap();
		let service = ProcessService::oneshot("echo", &["testing"]);

		let instance = manager.add_service(&mut rt, service, true);
		rt.poll_service_ready(&mut manager, &instance).unwrap(); // wait for service to complete
		assert_eq!(instance.borrow().state(), ServiceState::Completed);
	}
}
