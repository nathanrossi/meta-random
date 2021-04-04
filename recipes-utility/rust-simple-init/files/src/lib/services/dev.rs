use std::path::PathBuf;
use std::process::Command;
use super::super::*;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;

pub enum DeviceManagerState
{
	Stopped,
	Starting,
	Running,
}

pub struct DeviceManagerService
{
	state : DeviceManagerState,
	aliases : kmod::AliasMap,
}

impl DeviceManagerService
{
	pub fn new() -> Self
	{
		return Self {
				state : DeviceManagerState::Stopped,
				aliases : kmod::AliasMap::from_system(),
			};
	}

	pub fn load_module(&self, runtime : &mut Runtime, devpath : &str, name : &str)
	{
		if let Some(name) = self.aliases.resolve(name) {
			// check if module is already loaded
			if kmod::module_is_loaded(name) {
				runtime.logger.service_log("dev",
					&format!("device {}, module {} already loaded", devpath, name));
				return;
			}

			// TODO: change this to not be blocking
			if let Ok(output) = Command::new("/sbin/modprobe").arg(name).output() {
				if output.status.success() {
					runtime.logger.service_log("dev",
						&format!("device {}, module {} loaded", devpath, name));
				} else {
					runtime.logger.service_log("dev",
						&format!("device {}, module {} load failed", devpath, name));
				}
			}
		}
	}
}

impl Service for DeviceManagerService
{
	fn setup(&mut self, _runtime : &mut Runtime) {}

	fn state(&self) -> ServiceState
	{
		match self.state {
			DeviceManagerState::Stopped => ServiceState::Inactive,
			DeviceManagerState::Starting => ServiceState::Starting,
			DeviceManagerState::Running => ServiceState::Running,
		}
	}

	fn start(&mut self, runtime : &mut Runtime)
	{
		self.state = DeviceManagerState::Starting;

		// scan for modalias in /sys
		let mut paths = vec![PathBuf::from("/sys/devices")];
		loop {
			if let Some(path) = paths.pop() {
				if let Ok(entries) = std::fs::read_dir(path) {
					for i in entries {
						if let Ok(entry) = i {
							// check if directory
							if let Ok(filetype) = entry.file_type() {
								if filetype.is_file() && entry.file_name() == "modalias" {
									if let Some(devpath) = entry.path().parent() {
										if let Some(alias) = sysfs::read_file(entry.path()) {
											self.load_module(runtime, devpath.to_str().unwrap_or("<unknown>"), &alias);
										}
									}
								} else if filetype.is_dir() {
									paths.push(entry.path());
								}
							}
						}
					}
				}
			} else {
				break;
			}
		}

		runtime.logger.service_log("dev", "started");
		self.state = DeviceManagerState::Running;
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
		self.state = DeviceManagerState::Stopped;
	}

	fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
	{
		match event {
			ServiceEvent::Device(event) => {
				if event.udev {
					return false;
				}

				if let Some(action) = event.properties.get("ACTION") {
					if action != "add" {
						return false;
					}

					if let Some(alias) = event.get("MODALIAS") {
						self.load_module(runtime, event.devpath().unwrap_or("<unknown>"), alias);
					}
				}
			}
			_ => {}
		}
		return false;
	}
}
