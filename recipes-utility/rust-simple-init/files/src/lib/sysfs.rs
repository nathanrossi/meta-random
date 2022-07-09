use std::io;
use std::io::{BufRead, Read, BufReader};

pub trait SysfsEntryParsable<T>
{
	fn parse(line : &str) -> Option<T>;
}

pub struct SysfsEntryIter<R, T>
{
	file : std::io::BufReader<R>,
	evaluator : std::marker::PhantomData<T>,
}

impl<R: Read, T: SysfsEntryParsable<T>> Iterator for SysfsEntryIter<R, T>
{
	type Item = T;

	fn next(&mut self) -> Option<T>
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

				if let Some(entry) = T::parse(&buffer.trim()) {
					return Some(entry);
				}
			} else {
				return None;
			}
		}
	}
}

impl<T: SysfsEntryParsable<T>> SysfsEntryIter<std::fs::File, T>
{
	pub fn from_file(path : &str) -> io::Result<SysfsEntryIter<std::fs::File, T>>
	{
		return Ok(SysfsEntryIter {
				file : BufReader::new(std::fs::File::open(path)?),
				evaluator : std::marker::PhantomData,
			});
	}
}

impl<T: SysfsEntryParsable<T>> SysfsEntryIter<&[u8], T>
{
	pub fn from_string(s : &str) -> SysfsEntryIter<&[u8], T>
	{
		return SysfsEntryIter {
				file : BufReader::new(s.as_bytes()),
				evaluator : std::marker::PhantomData,
			};
	}
}

pub fn read_file<P: AsRef<std::path::Path>>(path : P) -> Option<String>
{
	if let Ok(content) = std::fs::read_to_string(path) {
		return Some(content);
	}
	return None;
}

pub fn read_line_file<P: AsRef<std::path::Path>>(path : P) -> Option<String>
{
	if let Ok(content) = std::fs::read_to_string(path) {
		if let Some(line) = content.lines().next() {
			return Some(line.to_string());
		}
	}
	return None;
}
