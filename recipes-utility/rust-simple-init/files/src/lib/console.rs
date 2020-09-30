use std::process::{Command, Child, ExitStatus};
use std::path::Path;
use super::*;
use super::runtime::{Service, ServiceState, ServiceContext, Runtime};

pub struct ConsoleService
{
	name : String,
	baud : u32,
	respawn : bool,
	process : Option<Child>,
}

impl ConsoleService
{
	pub fn new(name : &str, baud : u32, respawn : bool) -> Self
	{
		return Self { name : name.to_owned(), baud : baud, respawn : respawn, process : None };
	}
}

impl Service for ConsoleService
{
	fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		context.register_device_subsystem("tty");
	}

	fn state(&self) -> ServiceState
	{
		return ServiceState::Unknown;
	}

	fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		let devpath = Path::new("/dev").join(&self.name);
		if devpath.exists() {
			if let Some(path) = devpath.to_str() {
				println!("[console:{}] exists at path {}", self.name, path);
				if let Some(_) = self.process {
					return;
				}

				if let Ok(child) = Command::new("/sbin/getty").args(&["-i", "-L", &self.baud.to_string(), path]).spawn() {
					println!("[console:{}] started serial login console", self.name);
					context.register_child(&child);
					self.process = Some(child);
				}
			}
		}
	}

	fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		if let Some(child) = &mut self.process {
			if let Ok(_) = child.kill() {
				if let Ok(_) = child.wait() {
					self.process = None;
				}
			}
		}
	}

	fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : nix::unistd::Pid, status : ExitStatus)
	{
		if let Some(child) = &self.process {
			if pid == nix::unistd::Pid::from_raw(child.id() as i32) {
				self.process = None;
				if self.respawn {
					self.start(runtime, context);
				}
			}
		}
	}

	fn device_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, event : &uevent::EventData)
	{
		if event.udev {
			return;
		}

		if let Some(action) = event.properties.get("ACTION") {
			if action == "add" || action == "remove" {
				let added = action == "add";

				if let Some(subsys) = event.properties.get("SUBSYSTEM") {
					if subsys == "tty" {
						if let Some(devname) = event.properties.get("DEVNAME") {
							let mut name : &str = &devname;
							if devname.starts_with("/dev/") {
								name = &devname[5..];
							}
							if self.name == name {
								if added {
									self.start(runtime, context);
								} else {
									self.stop(runtime, context);
								}
							}
						}
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
	use runtime::ServiceManager;

	#[test]
	fn test()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new().unwrap();
		let mut service = ConsoleService::new("ttyUSB0", 115200, true);

		manager.add_service(&rt, service, false);

		// TODO: cannot really do emulation of devices for integration testing
		// rt.poll(&mut manager);
	}
}
