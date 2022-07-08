use std::process::{Command, Child};
use std::path::Path;
use super::super::*;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;

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
	fn setup(&mut self, _runtime : &mut Runtime) {}

	fn state(&self) -> ServiceState
	{
		return ServiceState::Unknown;
	}

	fn start(&mut self, runtime : &mut Runtime)
	{
		let devpath = Path::new("/dev").join(&self.name);
		if devpath.exists() {
			if let Some(path) = devpath.to_str() {
				runtime.logger.service_log(&format!("console:{}", self.name), &format!("exists at path {}", path));
				if let Some(_) = self.process {
					return;
				}

				// getty expects the 'tty...' subpath of /dev/, so use the name of the tty
				if let Ok(child) = Command::new("/sbin/getty").args(&["-i", "-L", &self.baud.to_string(), &self.name]).spawn() {
					runtime.logger.service_log(&format!("console:{}", self.name), "started serial login console");
					self.process = Some(child);
				}
			}
		}
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
		if let Some(child) = &mut self.process {
			if let Ok(_) = child.kill() {
				if let Ok(_) = child.wait() {
					self.process = None;
				}
			}
		}
	}

	fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
	{
		match event {
			ServiceEvent::ProcessExited(pid, status) => {
				if let Some(child) = &self.process {
					if child.id() == pid {
						runtime.logger.service_log(&format!("console:{}", self.name), &format!("getty exited ({})", status));
						self.process = None;
						if self.respawn {
							self.start(runtime);
						}
						return true;
					}
				}
			}
			ServiceEvent::Device(event) => {
				if event.udev {
					return false;
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
											self.start(runtime);
										} else {
											self.stop(runtime);
										}
									}
									return true;
								}
							}
						}
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
	fn test()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new_default_logger().unwrap();
		let service = ConsoleService::new("ttyUSB0", 115200, true);

		manager.add_service(&mut rt, service, false);

		// TODO: cannot really do emulation of devices for integration testing
		// rt.poll(&mut manager);
	}
}
