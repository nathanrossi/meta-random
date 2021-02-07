use std::path::Path;
use std::net::Ipv4Addr;
use std::process::{Command, Child};
use super::super::*;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;

pub enum Config
{
	DHCP,
	StaticIpv4(Ipv4Addr, u32, Option<Ipv4Addr>),
	DHCPD(Ipv4Addr, Ipv4Addr),
	WPASupplicant(String),
}

enum State
{
	None,
	Ready,
	LinkSetup(Child),
	LinkStaticIpv4(Child),
	LinkDHCP(Child),
	LinkDHCPD(Child),
	WPASupplicant(Child),
}

struct ConfigState
{
	config : Config,
	state : State,
}

pub struct NetworkDeviceService
{
	name : String,
	configs : Vec<ConfigState>,
}

impl NetworkDeviceService
{
	pub fn new(name : &str) -> Self
	{
		return Self { name : name.to_owned(), configs : Vec::new() };
	}


	pub fn iface_available(iface : &str) -> bool
	{
		let ifacedir = Path::new("/sys/class/net").join(iface);
		return ifacedir.exists();
	}

	pub fn add(&mut self, config : Config) -> &mut Self
	{
		self.configs.push(ConfigState { config : config, state : State::None });
		return self;
	}

	pub fn available(&self) -> bool
	{
		return NetworkDeviceService::iface_available(&self.name);
	}

	fn startup_step(&mut self, runtime : &mut Runtime)
	{
		for c in &mut self.configs {
			match c.state {
				State::Ready => { continue; } // this state is done, move to next
				State::None => {
						if let Ok(child) = Command::new("/sbin/ip").args(&["link", "set", "dev", &self.name, "up"]).spawn() {
							runtime.logger.service_log(&format!("net:{}", self.name), "link bringup");
							c.state = State::LinkSetup(child);
						}
					}
				State::LinkSetup(_) => {
						match &c.config {
							Config::StaticIpv4(host, prefix, _) => {
									let fmtaddr = format!("{}/{}", host, prefix);
									if let Ok(child) = Command::new("/sbin/ip").args(&["addr", "add", &fmtaddr, "dev", &self.name]).spawn() {
										runtime.logger.service_log(&format!("net:{}", self.name), "link addr static setup");
										c.state = State::LinkStaticIpv4(child);
									}
								}
							Config::DHCP => {
									if let Ok(child) = Command::new("/sbin/udhcpc").args(&["-f", "-i", &self.name]).spawn() {
										runtime.logger.service_log(&format!("net:{}", self.name), "link dhcp");
										c.state = State::LinkDHCP(child);
									} else {
										runtime.logger.service_log(&format!("net:{}", self.name), "failed to start udhcpc");
									}
								}
							Config::DHCPD(start, end) => {
									// create config
									let configpath = format!("/var/run/dhcp.{}.conf", self.name);
									runtime.logger.service_log(&format!("net:{}", self.name), &format!("link dhcpd config path {}", configpath));
									if let Ok(_) = std::fs::write(&configpath, [
											&format!("start {}", start),
											&format!("end {}", end),
											&format!("interface {}", self.name),
											&format!("lease_file /var/run/dhcp.{}.leases", self.name),
											"option subnet 255.255.255.252",
											"option lease 3600",
											].join("\n")) {
										if let Ok(child) = Command::new("/usr/sbin/udhcpd").arg(configpath).spawn() {
											runtime.logger.service_log(&format!("net:{}", self.name), "link dhcpd");
											c.state = State::LinkDHCPD(child);
										} else {
											runtime.logger.service_log(&format!("net:{}", self.name), "failed to start udhcpd");
										}
									}
								}
							Config::WPASupplicant(path) => {
									// copy config path to temporary location
									let configpath = format!("/var/run/wpa.{}.conf", self.name);
									runtime.logger.service_log(&format!("net:{}", self.name), &format!("link wpa supplicant config path {}", configpath));
									if std::path::Path::new(path).exists() {
										if let Ok(_) = std::fs::copy(&path, &configpath) {
											runtime.logger.service_log(&format!("net:{}", self.name), &format!("using wpa config from {}", path));
										}
									} else {
										runtime.logger.service_log(&format!("net:{}", self.name), &format!("wpa config {} does not exist, creating empty config", path));
										if let Ok(_) = std::fs::write(&configpath, "") {
											runtime.logger.service_log(&format!("net:{}", self.name), "failed to created empty wpa config");
										}
									}

									// start wpa_supplicant process on the interface
									if let Ok(child) = Command::new("/usr/sbin/wpa_supplicant")
											.args(&["-c", &configpath, "-i", &self.name]).spawn() {
										runtime.logger.service_log(&format!("net:{}", self.name), "starting wpa supplicant");
										c.state = State::WPASupplicant(child);
									} else {
										runtime.logger.service_log(&format!("net:{}", self.name), "failed to start wpa supplicant");
									}
								}
						}
					}
				State::LinkStaticIpv4(_) => { c.state = State::Ready; }
				State::LinkDHCP(_) => { continue; }
				State::LinkDHCPD(_) => { continue; }
				State::WPASupplicant(_) => { continue; }
			}
		}
	}
}

impl Service for NetworkDeviceService
{
	fn setup(&mut self, _runtime : &mut Runtime) {}

	fn state(&self) -> ServiceState
	{
		return ServiceState::Unknown;
	}

	fn start(&mut self, runtime : &mut Runtime)
	{
		if self.available() {
			// safely check state first
			for c in &self.configs {
				if let State::None = c.state {
					continue;
				}
				return; // don't start if one or more states are not None
			}
			self.startup_step(runtime);
		}
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
		// TODO: safe bringdown?
	}

	fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
	{
		match event {
			ServiceEvent::ProcessExited(_pid, _status) => {
				// TODO: actually check the return code and pid
				self.startup_step(runtime);
				return true;
			}
			ServiceEvent::Device(event) => {
				if event.udev {
					return false;
				}

				if let Some(action) = event.properties.get("ACTION") {
					if !(action == "add" || action == "remove") {
						return false;
					}

					let added = action == "add";
					if let Some(subsys) = event.properties.get("SUBSYSTEM") {
						if subsys == "net" {
							return false;
						}
					}
					if let Some(name) = event.properties.get("INTERFACE") {
						if &self.name == name {
							runtime.logger.service_log(&format!("net:{}", self.name),
								&format!("device was {}", match added { true => "added", false => "removed" }));
							if added {
								self.start(runtime);
							} else {
								self.stop(runtime);
							}
							return true;
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
		let mut service = NetworkDeviceService::new("usb0");
		service.add(Config::StaticIpv4(Ipv4Addr::new(169, 254, 1, 1), 30, None));

		manager.add_service(&mut rt, service, false);

		// TODO: cannot really do emulation of devices for integration testing
		// rt.poll(&mut manager);
	}
}
