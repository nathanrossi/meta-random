use std::io;
use std::io::BufRead;

pub struct MountIter
{
	file: std::io::BufReader<std::fs::File>,
	buffer: String,
}

impl Iterator for MountIter
{
	type Item = (String, String, String, String);

	fn next(&mut self) -> Option<Self::Item>
	{
		loop {
			self.buffer.clear();
			if let Ok(count) = self.file.read_line(&mut self.buffer) {
				if count == 0 {
					return None; // EOF
				}

				if self.buffer.len() == 0 {
					continue;
				}

				let mut parts = self.buffer.split(" ");
				let device = parts.next();
				let point = parts.next();
				let fstype = parts.next();
				let options = parts.next();
				if device == None || point == None || fstype == None || options == None {
					continue;
				}

				return Some((
						device.unwrap().to_owned(),
						point.unwrap().to_owned(),
						fstype.unwrap().to_owned(),
						options.unwrap().to_owned()));
			} else {
				return None;
			}
		}
	}
}

pub fn mounts() -> io::Result<MountIter>
{
	return Ok(MountIter { file: std::io::BufReader::new(std::fs::File::open("/proc/mounts")?), buffer: String::with_capacity(512) });
}

pub fn mount_available(point : &str, fstype : Option<&str>) -> bool
{
	if let Ok(mut entries) = mounts() {
		return entries.any(|(_, epoint, efstype, _)| {
				if epoint != point {
					return false;
				}
				if let Some(t) = fstype {
					if t != efstype {
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

	#[test]
	fn test_mount_iterates()
	{
		assert_ne!(mounts().unwrap().count(), 0);
	}
}
