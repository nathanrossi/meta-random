use std::path::Path;
use std::net::Ipv4Addr;
use std::process::{Command, Child};
use super::*;
use super::runtime::{Service, ServiceState, ServiceContext, Runtime};

pub enum Config
{
	None,
	DHCP,
	StaticIpv4(Ipv4Addr, u32, Option<Ipv4Addr>),
}

enum State
{
	None,
	Ready,
	LinkSetup(Child),
	LinkStaticIpv4(Child),
	LinkDHCP(Child),
}

pub struct NetworkDeviceService
{
	name : String,
	config : Config,
	state : State,
}

impl NetworkDeviceService
{
	pub fn new(name : &str, config : Config) -> Self
	{
		return Self { name : name.to_owned(), config : config, state : State::None };
	}

	pub fn iface_available(iface : &str) -> bool
	{
		let ifacedir = Path::new("/sys/class/net").join(iface);
		return ifacedir.exists();
	}

	pub fn available(&self) -> bool
	{
		return NetworkDeviceService::iface_available(&self.name);
	}

	fn startup_step(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		match self.state {
			State::None => {
					if let Ok(child) = Command::new("/sbin/ip").args(&["link", "set", "dev", &self.name, "up"]).spawn() {
						println!("[net:{}] link bringup", self.name);
						context.register_child(&child);
						self.state = State::LinkSetup(child);
					}
				}
			State::LinkSetup(_) => {
					match self.config {
						Config::StaticIpv4(host, prefix, _) => {
								let fmtaddr = format!("{}/{}", host, prefix);
								if let Ok(child) = Command::new("/sbin/ip").args(&["addr", "add", &fmtaddr, "dev", &self.name]).spawn() {
									println!("[net:{}] link addr static setup", self.name);
									context.register_child(&child);
									self.state = State::LinkStaticIpv4(child);
								}
							}
						Config::DHCP => {
								if let Ok(child) = Command::new("/sbin/udhcpc").args(&["-f", "-i", &self.name]).spawn() {
									println!("[net:{}] link dhcp", self.name);
									context.register_child(&child);
									self.state = State::LinkDHCP(child);
								}
							}
						_ => {}
					}
				}
			State::LinkStaticIpv4(_) => {
					self.state = State::Ready;
				}
			State::LinkDHCP(_) => {
					self.state = State::Ready;
				}
			_ => {}
		}
	}
}

impl Service for NetworkDeviceService
{
	fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		context.register_device_subsystem("net");
	}

	fn state(&self) -> ServiceState
	{
		return ServiceState::Unknown;
	}

	fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		if self.available() {
			if let State::None = self.state {
				self.startup_step(runtime, context);
			}
		}
	}

	fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		// TODO: safe bringdown?
		self.state = State::None;
	}

	fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : nix::unistd::Pid, status : std::process::ExitStatus)
	{
		// TODO: actually check the return code and pid
		self.startup_step(runtime, context);
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
					if subsys == "net" {
						if let Some(name) = event.properties.get("ID_NET_NAME_PATH") {
							if &self.name == name {
								println!("[net:{}] device was {}", self.name, match added { true => "added", false => "removed"});
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
		let mut service = NetworkDeviceService::new("usb0", Config::StaticIpv4(Ipv4Addr::new(169, 254, 1, 1), 30, None));

		manager.add_service(&rt, service, false);

		// TODO: cannot really do emulation of devices for integration testing
		// rt.poll(&mut manager);
	}
}
