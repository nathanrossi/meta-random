use std::io;
use std::fs::File;
use std::path::Path;

pub struct Logger<'a>
{
	pub prefix : Option<String>,
	writers : Vec<Box<dyn io::Write + 'a>>,
}

impl<'a> Logger<'a>
{
	pub fn new() -> Self
	{
		return Self { prefix : None, writers : Vec::new() };
	}

	pub fn add(&mut self, write : impl io::Write + 'a)
	{
		self.writers.push(Box::new(write));
	}

	pub fn add_file<P: AsRef<Path>>(&mut self, path : P) -> io::Result<()>
	{
		// create the path to the output file
		if let Some(parent) = path.as_ref().parent() {
			std::fs::create_dir_all(parent).ok();
		}

		// create the file object
		let output = File::create(path)?;
		self.add(output);
		return Ok(());
	}

	fn output(&mut self, data : &str)
	{
		for f in self.writers.iter_mut() {
			if let Ok(_) = f.write(data.as_bytes()) {
			}
		}
	}

	pub fn log(&mut self, info : &str)
	{
		let line : String;
		if let Some(prefix) = &self.prefix {
			line = format!("[{}] {}\n", prefix, info);
		} else {
			line = format!("{}\n", info);
		}
		self.output(&line);
	}

	pub fn service_log(&mut self, service : &str, info : &str)
	{
		self.log(&format!("[{}] {}", service, info));
	}
}

