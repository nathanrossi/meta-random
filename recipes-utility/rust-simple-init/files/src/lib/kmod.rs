use std::io;
use std::io::{Read, BufRead, BufReader};
use super::sysfs::*;

#[derive(Debug)]
pub struct ModuleEntry
{
	name : String,
	size : usize,
}

impl SysfsEntryParsable<ModuleEntry> for ModuleEntry
{
	fn parse(line : &str) -> Option<Self>
	{
		let mut parts = line.split(" ");
		let name = parts.next();
		let sizestr = parts.next();
		parts.next(); // instances
		let dependencies = parts.next(); // dependencies
		if name == None || sizestr == None || dependencies == None {
			return None;
		}

		let mut size : usize = 0;
		if let Ok(value) = sizestr.unwrap().parse::<usize>() {
			size = value;
		}

		return Some(ModuleEntry { name : name.unwrap().to_owned(), size });
	}
}

pub fn modules() -> io::Result<SysfsEntryIter<std::fs::File, ModuleEntry>>
{
	return SysfsEntryIter::from_file("/proc/modules");
}

pub fn modules_from_string(data : &str) -> SysfsEntryIter<&[u8], ModuleEntry>
{
	return SysfsEntryIter::from_string(data);
}

pub fn module_is_loaded(module : &str) -> bool
{
	if let Ok(modules) = modules() {
		for i in modules {
			if i.name == module {
				return true;
			}
		}
	}
	return false;
}

struct AliasMapEntry
{
	pattern : String,
	module : String,
}

impl AliasMapEntry
{
	fn matches(&self, name : &str) -> bool
	{
		// match string, allowing for wildcard "*". Wildcard is lazy, >1 character
		let mut pattern = self.pattern.chars();
		let mut wildnext = None;
		for v in name.chars() {
			if let Some(next) = wildnext {
				if v == next {
					wildnext = None;
				}
			} else {
				if let Some(p) = pattern.next() {
					if p == '*' {
						if let Some(next) = pattern.next() {
							wildnext = Some(next);
						} else {
							return true; // will match anything else
						}
					} else if p != v {
						return false;
					}
				}
			}
		}
		return true;
	}
}

pub struct AliasMap
{
	entries : Vec<AliasMapEntry>,
}

impl AliasMap
{
	pub fn resolve(&self, name : &str) -> Option<&str>
	{
		// search the map for any entries that match completly
		for i in &self.entries {
			if i.matches(name) {
				return Some(&i.module);
			}
		}
		return None;
	}

	pub fn from_reader<R: Read>(mut reader : BufReader<R>) -> io::Result<Self>
	{
		let mut map = AliasMap { entries : Vec::new() };
		let mut buffer : String = String::with_capacity(512);
		loop {
			buffer.clear();
			let count = reader.read_line(&mut buffer)?;
			if count == 0 {
				break; // EOF
			}

			if buffer.starts_with("alias") {
				let mut parts = buffer.trim().split(" ");
				if let Some(_) = parts.next() {
					if let Some(pattern) = parts.next() {
						if let Some(module) = parts.next() {
							map.entries.push(AliasMapEntry { pattern : pattern.to_owned(), module : module.to_owned() });
						}
					}
				}
			}
		}
		return Ok(map);
	}

	pub fn from_path<P: AsRef<std::path::Path>>(path : P) -> io::Result<Self>
	{
		return Self::from_reader(BufReader::new(std::fs::File::open(path)?));
	}

	pub fn from_string(s : &str) -> io::Result<Self>
	{
		return Self::from_reader(BufReader::new(s.as_bytes()));
	}

	pub fn from_system() -> Self
	{
		// get /lib/modules/(uname -r)/modules.alias
		if let Ok(uname) = nix::sys::utsname::uname() {
			let path = std::path::Path::new("/lib/modules").join(uname.release()).join("modules.alias");
			if path.exists() {
				if let Ok(map) = Self::from_path(path) {
					return map;
				}
			}
		}
		return AliasMap { entries : Vec::new() };
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn proc_modules_parsing()
	{
		let mut v = modules_from_string(
			"usb_common 16384 3 uvcvideo,xhci_hcd,usbcore, Live 0x0000000000000000\n\
			wmi 36864 2 intel_wmi_thunderbolt,wmi_bmof, Live 0x0000000000000000\n\
			battery 20480 1 thinkpad_acpi, Live 0x0000000000000000\n\
			i2c_hid 32768 0 - Live 0x0000000000000000\n");

		if let Some(e) = v.next() {
			assert_eq!(e.name, "usb_common");
			assert_eq!(e.size, 16384);
		} else {
			assert!(false);
		}

		if let Some(e) = v.next() {
			assert_eq!(e.name, "wmi");
			assert_eq!(e.size, 36864);
		} else {
			assert!(false);
		}

		if let Some(e) = v.next() {
			assert_eq!(e.name, "battery");
			assert_eq!(e.size, 20480);
		} else {
			assert!(false);
		}

		if let Some(e) = v.next() {
			assert_eq!(e.name, "i2c_hid");
			assert_eq!(e.size, 32768);
		} else {
			assert!(false);
		}

		assert!(v.next().is_none());
	}

	#[test]
	fn module_alias_matching()
	{
		let entry = AliasMapEntry { pattern : "usb:v*p*d*dc*dsc*dp*ic03isc*ip*in*".to_string(), module : "usbhid".to_string() };
		assert_eq!(entry.matches("usb:v0627p0001d0000dc00dsc00dp00ic03isc01ip01in00"), true);
		assert_eq!(entry.matches("hid:b0003g0001v00000627p00000001"), false);
	}

	#[test]
	fn modules_alias_parsing()
	{
		let v = AliasMap::from_string(
			"# Aliases extracted from modules themselves.\n\
			alias cpu:type:x86,ven0000fam0006mod0086:feature:* intel_uncore\n\
			alias blowfish-asm blowfish_x86_64\n\
			alias acpi*:APP0002:* apple_bl\n\
			alias pci:v00001002d00007151sv*sd*bc*sc*i* radeon\n\
			alias usb:v*p*d*dc*dsc*dp*ic03isc*ip*in* usbhid\n\
			").unwrap();

		assert_eq!(v.resolve("usb:v0627p0001d0000dc00dsc00dp00ic03isc01ip01in00"), Some("usbhid"));
		assert_eq!(v.resolve("bad"), None);
	}

	#[test]
	fn modules_alias_from_system()
	{
		let map = AliasMap::from_system();
		assert!(map.entries.len() != 0);
	}
}
