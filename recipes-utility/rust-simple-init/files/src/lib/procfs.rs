use std::io;
use std::io::{BufRead, Read, BufReader};

#[derive(Debug)]
pub struct MountEntry
{
	fstype : String,
	device : String,
	point : String,
	options : String,
}

impl MountEntry
{
	pub fn new(fstype : &str, device : &str, point : &str, options : &str) -> Self
	{
		return MountEntry {
				fstype : fstype.to_owned(),
				device : device.to_owned(),
				point : point.to_owned(),
				options :options.to_owned(),
			};
	}
}

pub struct MountIter<R>
{
	file: std::io::BufReader<R>,
}

impl<R: Read> Iterator for MountIter<R>
{
	type Item = MountEntry;

	fn next(&mut self) -> Option<Self::Item>
	{
		let mut buffer : String = String::with_capacity(512);
		loop {
			buffer.clear();
			if let Ok(count) = self.file.read_line(&mut buffer) {
				if count == 0 {
					return None; // EOF
				}

				if buffer.len() == 0 {
					continue;
				}

				let mut parts = buffer.split(" ");
				let device = parts.next();
				let point = parts.next();
				let fstype = parts.next();
				let options = parts.next();
				if device == None || point == None || fstype == None || options == None {
					continue;
				}

				return Some(MountEntry::new(fstype.unwrap(), device.unwrap(), point.unwrap(), options.unwrap()));
			} else {
				return None;
			}
		}
	}
}

pub fn mounts() -> io::Result<MountIter<std::fs::File>>
{
	return Ok(MountIter { file : BufReader::new(std::fs::File::open("/proc/self/mounts")?) });
}

pub fn mounts_from_string(data : &str) -> MountIter<&[u8]>
{
	return MountIter { file : BufReader::new(data.as_bytes()) };
}

pub fn mounted(point : &str, device : Option<&str>, fstype : Option<&str>) -> bool
{
	if let Ok(mut entries) = mounts() {
		return entries.any(|e| {
				if e.point != point {
					return false;
				}
				if let Some(d) = device {
					if d != e.device {
						return false;
					}
				}
				if let Some(t) = fstype {
					if t != e.fstype {
						return false;
					}
				}
				return true;
			});
	}
	return false;
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::option::Option;

	#[test]
	fn iterates()
	{
		assert_ne!(mounts().unwrap().count(), 0);
	}

	#[test]
	fn parsing()
	{
		let mut v = mounts_from_string(
			"udev /dev devtmpfs rw,nosuid,noexec,relatime,size=3938972k,nr_inodes=984743,mode=755 0 0\n\
			sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0\n\
			proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0\n\
			/dev/nvme0n1p2 / ext4 rw,relatime,errors=remount-ro 0 0\n");

		if let Some(e) = v.next() {
			assert_eq!(e.fstype, "devtmpfs");
			assert_eq!(e.device, "udev");
			assert_eq!(e.point, "/dev");
			assert_eq!(e.options, "rw,nosuid,noexec,relatime,size=3938972k,nr_inodes=984743,mode=755");
		} else {
			assert!(false);
		}

		if let Some(e) = v.next() {
			assert_eq!(e.fstype, "sysfs");
			assert_eq!(e.device, "sysfs");
			assert_eq!(e.point, "/sys");
			assert_eq!(e.options, "rw,nosuid,nodev,noexec,relatime");
		} else {
			assert!(false);
		}

		if let Some(e) = v.next() {
			assert_eq!(e.fstype, "proc");
			assert_eq!(e.device, "proc");
			assert_eq!(e.point, "/proc");
			assert_eq!(e.options, "rw,nosuid,nodev,noexec,relatime");
		} else {
			assert!(false);
		}

		if let Some(e) = v.next() {
			assert_eq!(e.fstype, "ext4");
			assert_eq!(e.device, "/dev/nvme0n1p2");
			assert_eq!(e.point, "/");
			assert_eq!(e.options, "rw,relatime,errors=remount-ro");
		} else {
			assert!(false);
		}

		assert!(v.next().is_none());
	}
}
