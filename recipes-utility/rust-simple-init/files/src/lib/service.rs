use std::process::{Command, Child, ExitStatus};
use super::*;
use super::runtime::{Service, ServiceState, ServiceContext, Runtime};

enum ProcessState
{
	Stopped,
	Waiting,
	Running(Child),
	Completed,
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

	fn check_devices(&self) -> bool
	{
		for i in &self.devices {
			let path = std::path::Path::new(&i);
			if !path.exists() {
				return false;
			}
		}
		return true;
	}

	fn check_state(&mut self, context : &mut ServiceContext)
	{
		if let ProcessState::Stopped = self.state {
			if !self.devices.is_empty() {
				self.state = ProcessState::Waiting;
			}
		}

		if let ProcessState::Waiting = self.state {
			if self.check_devices() {
				return;
			}
		}

		if let Ok(child) = self.command.spawn() {
			println!("[process] starting {:?}", self.command);
			context.register_child(&child);
			self.state = ProcessState::Running(child);
		}
	}
}

impl Service for ProcessService
{
	fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		context.register_device_subsystem("tty");
	}

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
			_ => { return ServiceState::Inactive; }
		}
	}

	fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		self.check_state(context);
	}

	fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		if let ProcessState::Running(child) = &mut self.state {
			if let Ok(_) = child.kill() {
				if let Ok(_) = child.wait() {
					self.state = ProcessState::Stopped;
				}
			}
		}
	}

	fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : nix::unistd::Pid, status : ExitStatus)
	{
		if let ProcessState::Running(child) = &mut self.state {
			if pid == nix::unistd::Pid::from_raw(child.id() as i32) {
				if self.oneshot {
					self.state = ProcessState::Completed; // TODO: error?
				} else {
					self.state = ProcessState::Stopped; // TODO: error?
				}
			}
		}
	}

	fn device_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, event : &uevent::EventData)
	{
		self.check_state(context);
	}
}

use std::io;
use configfs::usb::Gadget;

pub struct UsbGadgetService<'a>
{
	name : String,
	configfn : Box<dyn Fn() -> io::Result<Option<Gadget>> + 'a>,
	device : Option<Gadget>,
	failed : bool,
}

impl<'a> UsbGadgetService<'a>
{
	pub fn new(device : &str, config : impl Fn() -> io::Result<Option<Gadget>> + 'a) -> Self
	{
		return Self { name : device.to_owned(), configfn : Box::new(config), device : None, failed : false };
	}

	pub fn available(&self) -> bool
	{
		if let Ok(interfaces) = configfs::usb::Gadget::interfaces() {
			for interface in interfaces {
				if interface == self.name {
					return true;
				}
			}
		}
		return false;
	}
}

impl<'a> Service for UsbGadgetService<'a>
{
	fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
	}

	fn state(&self) -> ServiceState
	{
		if self.failed {
			return ServiceState::Error;
		}
		if let Some(_) = self.device {
			return ServiceState::Completed;
		}
		return ServiceState::Inactive;
	}

	fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		if self.failed {
			return;
		}

		if !self.available() {
			return
		}

		if let None = self.device {
			println!("[usbgadget] setting up device");
			let result = (&self.configfn)();
			if let Ok(d) = result {
				if let Some(device) = d {
					println!("[usbgadget] device configured");

					// attach to port
					println!("[usbgadget] usb gadget attaching to {}", self.name);
					if let Ok(_) = device.attach(&self.name) {
						self.device = Some(device);
					}
				}
			} else if let Err(e) = result {
				println!("[usbgadget] setting up device failed! (Err = {})", e);
				self.failed = true;
			}
		}
	}

	fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
	}

	fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : nix::unistd::Pid, status : ExitStatus) {}
	fn device_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, event : &uevent::EventData) {}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use runtime::ServiceManager;

	#[test]
	fn process()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new().unwrap();
		let mut service = ProcessService::new("cat", &[]);

		let mut instance = manager.add_service(&rt, service, true);
		rt.poll_service_ready(&mut manager, &instance).unwrap(); // wait for service to start
		assert_eq!(instance.borrow().state(), ServiceState::Running);

		instance.borrow_mut().stop(&rt);
		assert_eq!(instance.borrow().state(), ServiceState::Inactive);
	}
}
