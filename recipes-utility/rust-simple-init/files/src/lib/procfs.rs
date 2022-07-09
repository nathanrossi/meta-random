use std::io;
use std::path::Path;
use nix::unistd::Pid;
use super::sysfs::*;

#[derive(Debug)]
pub struct MountEntry
{
	fstype : String,
	device : String,
	point : String,
	options : String,
}

impl SysfsEntryParsable<MountEntry> for MountEntry
{
	fn parse(line : &str) -> Option<Self>
	{
		let mut parts = line.split(" ");
		let device = parts.next();
		let point = parts.next();
		let fstype = parts.next();
		let options = parts.next();
		if device == None || point == None || fstype == None || options == None {
			return None;
		}

		return Some(MountEntry {
				fstype : fstype.unwrap().to_owned(),
				device : device.unwrap().to_owned(),
				point : point.unwrap().to_owned(),
				options : options.unwrap().to_owned(),
			});
	}
}

#[allow(dead_code)]
pub fn mounts() -> io::Result<SysfsEntryIter<std::fs::File, MountEntry>>
{
	return SysfsEntryIter::from_file("/proc/self/mounts");
}

#[allow(dead_code)]
pub fn mounts_from_string(data : &str) -> SysfsEntryIter<&[u8], MountEntry>
{
	return SysfsEntryIter::from_string(data);
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

pub fn process_comm(pid : Pid) -> Option<String>
{
	let path = Path::new("/proc").join(pid.to_string()).join("comm");
	if path.exists() {
		return read_line_file(path);
	}
	return None;
}

#[cfg(test)]
mod tests {
	use super::*;

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

	#[test]
	fn test_comm()
	{
		assert!(process_comm(Pid::from_raw(1)).is_some());
	}
}
