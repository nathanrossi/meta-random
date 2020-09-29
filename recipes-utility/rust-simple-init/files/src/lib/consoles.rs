use std::io;
use std::path::Path;
use std::collections::HashMap;

struct Port
{
	baud : u32,
}

pub struct Manager
{
	ports : HashMap<String, Port>,
}

impl Manager
{
	pub fn new() -> Manager
	{
		return Manager { ports : HashMap::new() };
	}

	pub fn add(&mut self, name : &str, baud : u32) -> io::Result<()>
	{
		if !self.ports.contains_key(name) {
			self.ports.insert(name.to_owned(), Port { baud : baud });
			self.check(name)?;
		}
		return Ok(());
	}

	pub fn check(&self, name : &str) -> io::Result<()>
	{
		// check
		if Path::new("/dev").join(name).exists() {
			println!("node '{}' exists", name);
		}
		return Ok(());
	}

	pub fn check_uevent(&self, event : &std::collections::HashMap<String, String>) -> io::Result<()>
	{
		if let Some(action) = event.get("ACTION") {
			if action == "add" || action == "remove" {
				if let Some(subsys) = event.get("SUBSYSTEM") {
					if subsys == "tty" {
						if let Some(devname) = event.get("DEVNAME") {
							println!("change to tty '{}'", devname);
							self.check(&devname[5..])?;
						}
					}
				}
			}
		}
		return Ok(());
	}
}

// getty = Command::new("/sbin/getty").args(&["-i", "-w", "-L", "115200", "/dev/ttyGS0"]).spawn();

