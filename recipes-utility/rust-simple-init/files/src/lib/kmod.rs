use std::io;
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

#[allow(dead_code)]
pub fn modules() -> io::Result<SysfsEntryIter<std::fs::File, ModuleEntry>>
{
	return SysfsEntryIter::from_file("/proc/modules");
}

#[allow(dead_code)]
pub fn modules_from_string(data : &str) -> SysfsEntryIter<&[u8], ModuleEntry>
{
	return SysfsEntryIter::from_string(data);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn iterates()
	{
		assert_ne!(modules().unwrap().count(), 0);
	}

	#[test]
	fn parsing()
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
}
