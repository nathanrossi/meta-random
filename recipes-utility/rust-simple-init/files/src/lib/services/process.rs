use std::process::{Command, Child};
use super::super::*;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;

enum ProcessState
{
	Stopped,
	Waiting,
	Running(Child),
	Completed,
	Error(std::process::ExitStatus),
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
		if let ProcessState::Stopped = self.state {
			if !self.devices.is_empty() {
				self.state = ProcessState::Waiting;
			}
		}

		if let ProcessState::Waiting = self.state {
			if !self.check_devices(runtime) {
				return;
			}
		}

		if let Ok(child) = self.command.spawn() {
			runtime.logger.service_log("process", &format!("starting {:?}", self.command));
			self.state = ProcessState::Running(child);
		}
	}
}

impl Service for ProcessService
{
	fn setup(&mut self, _runtime : &mut Runtime) {}

	fn state(&self) -> ServiceState
	{
		match self.state {
			ProcessState::Running(_) => {
					if self.oneshot {
						return ServiceState::Starting;
					}
					return ServiceState::Running;
				}
			ProcessState::Completed => { return ServiceState::Completed; }
			ProcessState::Error(_) => { return ServiceState::Error; }
			_ => { return ServiceState::Inactive; }
		}
	}

	fn start(&mut self, runtime : &mut Runtime)
	{
		self.check_state(runtime);
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
		if let ProcessState::Running(child) = &mut self.state {
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
				if let ProcessState::Running(child) = &mut self.state {
					if child.id() == pid {
						if !status.success() {
							runtime.logger.service_log("process", &format!("failed to start, exit code = {:?}", status));
							self.state = ProcessState::Error(status);
							return true;
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
}
