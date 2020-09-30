use std::io;
use std::path::Path;
use super::uevent;

pub struct Manager
{
	pub name : String,
	pub baud : u32,
	getty : Option<std::process::Child>,
}

impl Manager
{
	pub fn new(name : &str, baud : u32) -> Manager
	{
		return Manager { name : name.to_owned(), baud : baud, getty : None };
	}

	pub fn check(&mut self) -> io::Result<()>
	{
		let devpath = Path::new("/dev").join(&self.name);

		println!("checking console: {}", self.name);
		if devpath.exists() {
			println!("node '{}' exists", self.name);
			if let Some(p) = &mut self.getty {
				if let Some(_) = p.try_wait()? {
					// previous getty is done
					self.getty = None;
				} else {
					return Ok(());
				}
			}

			if let Some(path) = devpath.to_str() {
				let child = std::process::Command::new("/sbin/getty").args(&["-i", "-w", "-L", &self.baud.to_string(), path]).spawn()?;
				self.getty = Some(child);
			}
		} else {
			self.kill()?;
		}
		return Ok(());
	}

	pub fn cleanup(&mut self) -> io::Result<()>
	{
		if let Some(p) = &mut self.getty {
			if let Some(_) = p.try_wait()? {
				// previous getty is done
				self.getty = None;
			}
		}
		return Ok(());
	}

	pub fn kill(&mut self) -> io::Result<()>
	{
		if let Some(p) = &mut self.getty {
			p.kill()?;
			p.wait()?;
			self.getty = None;
		}
		return Ok(());
	}

	pub fn uevent(&mut self, event : &uevent::EventData) -> io::Result<()>
	{
		if event.udev {
			return Ok(()); // skip udev events
		}

		if let Some(action) = event.properties.get("ACTION") {
			if action == "add" || action == "remove" {
				if let Some(subsys) = event.properties.get("SUBSYSTEM") {
					if subsys == "tty" {
						if let Some(devname) = event.properties.get("DEVNAME") {
							let mut name : &str = &devname;
							if devname.starts_with("/dev/") {
								name = &devname[5..];
							}
							if self.name == name {
								println!("change to tty '{}'", name);
								self.check()?;
							}
						}
					}
				}
			}
		}
		return Ok(());
	}
}

// getty = Command::new("/sbin/getty").args(&["-i", "-w", "-L", "115200", "/dev/ttyGS0"]).spawn();

