use std::path::Path;
use std::net::Ipv4Addr;
use std::process::{Command, Child};
use super::super::*;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;

pub enum Config
{
	LinkUp,
	DHCP,
	StaticIpv4(Ipv4Addr, u32, Option<Ipv4Addr>),
	DHCPD(Ipv4Addr, Ipv4Addr),
	WPASupplicant(String),
}

enum State
{
	None,
	Ready,
	Failed,
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

impl ConfigState
{
	pub fn begin(&mut self, device : &str, runtime : &mut Runtime)
	{
		match &mut self.state {
			State::None => {
					if let Ok(child) = Command::new("/sbin/ip").args(&["link", "set", "dev", device, "up"]).spawn() {
						runtime.logger.service_log(&format!("net:{}", device), &format!("link bringup (pid = {})", child.id()));
						self.state = State::LinkSetup(child);
					}
				}
			_ => { },
		}
	}

	pub fn check(&mut self, device : &str, runtime : &mut Runtime, pid : u32, _status : std::process::ExitStatus) -> bool
	{
		// TODO: handle the status of processes that exit
		match &mut self.state {
			State::Ready => { return false; } // this state is complete, nothing to do
			State::LinkSetup(child) => {
					if child.id() != pid {
						return false;
					}

					match &self.config {
						Config::LinkUp => {
								// Only bring the link up
								runtime.logger.service_log(&format!("net:{}", device), "link ready");
								self.state = State::Ready;
							}
						Config::StaticIpv4(host, prefix, _) => {
								let fmtaddr = format!("{}/{}", host, prefix);
								runtime.logger.service_log(&format!("net:{}", device), &format!("link configure static ipv4 '{}'", &fmtaddr));
								if let Ok(child) = Command::new("/sbin/ip").args(&["addr", "add", &fmtaddr, "dev", &device]).spawn() {
									runtime.logger.service_log(&format!("net:{}", device), "link addr static setup");
									self.state = State::LinkStaticIpv4(child);
								} else {
									runtime.logger.service_log(&format!("net:{}", device), "failed to configure static ipv4 address");
									self.state = State::Failed;
								}
							}
						Config::DHCP => {
								if let Ok(child) = Command::new("/sbin/udhcpc").args(&["-f", "-i", &device]).spawn() {
									runtime.logger.service_log(&format!("net:{}", device), "link dhcp");
									self.state = State::LinkDHCP(child);
								} else {
									runtime.logger.service_log(&format!("net:{}", device), "failed to start udhcpc");
									self.state = State::Failed;
								}
							}
						Config::DHCPD(start, end) => {
								// create config
								let configpath = format!("/var/run/dhcp.{}.conf", device);
								runtime.logger.service_log(&format!("net:{}", device), &format!("link dhcpd config path {}", configpath));
								if let Ok(_) = std::fs::write(&configpath, [
										&format!("start {}", start),
										&format!("end {}", end),
										&format!("interface {}", device),
										&format!("lease_file /var/run/dhcp.{}.leases", device),
										"option subnet 255.255.255.252",
										"option lease 3600",
										].join("\n")) {
									if let Ok(child) = Command::new("/usr/sbin/udhcpd").arg("-f").arg(configpath).spawn() {
										runtime.logger.service_log(&format!("net:{}", device), "link dhcpd");
										self.state = State::LinkDHCPD(child);
									} else {
										runtime.logger.service_log(&format!("net:{}", device), "failed to start udhcpd");
										self.state = State::Failed;
									}
								}
							}
						Config::WPASupplicant(path) => {
								// copy config path to temporary location
								let configpath = format!("/var/run/wpa.{}.conf", device);
								runtime.logger.service_log(&format!("net:{}", device), &format!("link wpa supplicant config path {}", configpath));
								if std::path::Path::new(path).exists() {
									if let Ok(_) = std::fs::copy(&path, &configpath) {
										runtime.logger.service_log(&format!("net:{}", device), &format!("using wpa config from {}", path));
									}
								} else {
									runtime.logger.service_log(&format!("net:{}", device), &format!("wpa config {} does not exist, creating empty config", path));
									if let Ok(_) = std::fs::write(&configpath, "") {
										runtime.logger.service_log(&format!("net:{}", device), "failed to created empty wpa config");
									}
								}

								// start wpa_supplicant process on the interface
								if let Ok(child) = Command::new("/usr/sbin/wpa_supplicant")
										.args(&["-c", &configpath, "-i", device]).spawn() {
									runtime.logger.service_log(&format!("net:{}", device), "starting wpa supplicant");
									self.state = State::WPASupplicant(child);
								} else {
									runtime.logger.service_log(&format!("net:{}", device), "failed to start wpa supplicant");
									self.state = State::Failed;
								}
							}
					}

					return true;
				}
			State::LinkStaticIpv4(child) => {
					if child.id() != pid {
						return false;
					}

					runtime.logger.service_log(&format!("net:{}", device), "link ready");
					self.state = State::Ready;
					return true;
				}
			// State::LinkDHCP(_) => ,
			// State::LinkDHCPD(_) => ,
			// State::WPASupplicant(_) => ,
			_ => {},
		}
		return false;
	}
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
			for c in &mut self.configs {
				c.begin(&self.name, runtime);
			}
		}
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
		// TODO: safe bringdown?
	}

	fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
	{
		match event {
			ServiceEvent::ProcessExited(pid, status) => {
				for c in &mut self.configs {
					if c.check(&self.name, runtime, pid, status) {
						return true;
					}
				}
				return false;
			}
			ServiceEvent::Device(event) => {
				if let Some(action) = event.properties.get("ACTION") {
					let added = action == "add";
					if !(action == "add" || action == "remove") {
						return false;
					}

					if let Some(subsys) = event.properties.get("SUBSYSTEM") {
						if subsys != "net" {
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
