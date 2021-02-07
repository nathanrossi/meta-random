use std::io;
use std::path::Path;
use super::super::procfs;

fn check_mount() -> bool
{
	// check to see if the configfs mount is available
	if !procfs::mounted("/sys/kernel/config", None, Some("configfs")) {
		println!("usbcfs: mounting");
		let mount = std::process::Command::new("/bin/mount")
			.args(&["-t", "configfs", "none", "/sys/kernel/config"])
			.stdout(std::process::Stdio::null())
			.stderr(std::process::Stdio::null())
			.status();
		if let Err(_) = mount {
			return false;
		}
	}

	// check to see if usb_gadget support is available
	if !Path::new("/sys/kernel/config/usb_gadget").is_dir() {
		return false;
	}
	return true;
}

fn count_entries<P: AsRef<Path>>(path : P, prefix : &str) -> io::Result<u32>
{
	let mut count : u32 = 0;
	if let Ok(entries) = std::fs::read_dir(path) {
		for i in entries {
			if let Ok(entry) = i {
				// check if directory
				if let Ok(filetype) = entry.file_type() {
					if !filetype.is_dir() {
						continue
					}
				} else {
					continue
				}

				if prefix.len() != 0 {
					// check for prefix
					if let Some(filename) = entry.path().file_name() {
						if let Some(s) = filename.to_str() {
							if s.len() >= 2 && s.starts_with(prefix) {
								count += 1;
							}
						}
					}
				} else {
					count += 1;
				}
			}
		}
	}
	return Ok(count);
}

fn read_dir_child_dirnames<P: AsRef<Path>>(path : P) -> io::Result<impl std::iter::Iterator<Item = String>>
{
	let entries = std::fs::read_dir(path)?;
	return Ok(entries
			.filter_map(|e : io::Result<std::fs::DirEntry>| {
					if let Ok(entry) = e {
						if let Ok(filetype) = entry.file_type() {
							if !filetype.is_dir() {
								return None;
							}
							let filename = entry.file_name();
							if let Some(name) = filename.to_str() {
								return Some(name.to_owned());
							}
						}
					}
					return None;
				})
		);
}

pub struct Gadget {
	name: String,
}

#[allow(dead_code)]
impl Gadget {
	pub fn create(serialnumber : &str, manufacturer : &str, product : &str) -> io::Result<Gadget>
	{
		if !check_mount() {
			return Err(io::Error::new(io::ErrorKind::NotFound, "Missing configfs userspace support"));
		}

		let root = Path::new("/sys/kernel/config/usb_gadget");
		// count gadgets (don't assume they are all prefixed with "g"
		let count = count_entries(root, "")?;

		let name = format!("g{}", count + 1);
		let base = root.join(&name);
		std::fs::create_dir_all(&base)?;

		// setup vendor/product id (use Linux Foundation's)
		std::fs::write(base.join("idVendor"), "0x1d6b")?;
		std::fs::write(base.join("idProduct"), "0x0104")?;

		// setup base strings (english)
		let strings = base.join("strings").join("0x409");
		std::fs::create_dir_all(&strings)?;

		std::fs::write(strings.join("serialnumber"), serialnumber)?;
		std::fs::write(strings.join("manufacturer"), manufacturer)?;
		std::fs::write(strings.join("product"), product)?;

		// functions
		let functions = base.join("functions");
		std::fs::create_dir_all(&functions)?;

		// configs
		let configs = base.join("configs");
		std::fs::create_dir_all(&configs)?;

		return Ok(Gadget { name : name });
	}

	pub fn path(&self) -> std::io::Result<std::path::PathBuf>
	{
		let base = Path::new("/sys/kernel/config/usb_gadget").join(&self.name);
		if !base.is_dir() {
			return Err(io::Error::new(io::ErrorKind::NotFound, "Gadget device does not exist"));
		}
		return Ok(base);
	}

	pub fn add_function(&self, function : &str, name : &str) -> io::Result<String>
	{
		let functions = self.path()?.join("functions");
		let base = format!("{}.{}", function, name);
		std::fs::create_dir_all(&functions)?;
		// TODO: if above succeeds but the need one fails, then the function is not available? (aka
		// module is missing or not configured in the kernel.
		std::fs::create_dir_all(&functions.join(&base))?;
		return Ok(base);
	}

	pub fn add_config(&self, name : &str, functions : &[&str]) -> io::Result<String>
	{
		let configs = self.path()?.join("configs");
		let count = count_entries(&configs, "c.")?;
		let config = format!("c.{}", count + 1);
		let base = configs.join(&config);
		std::fs::create_dir_all(&configs)?;
		std::fs::create_dir_all(&base)?;

		// setup base strings (english)
		let strings = base.join("strings").join("0x409");
		std::fs::create_dir_all(&strings)?;
		std::fs::write(strings.join("configuration"), name)?;

		// symlink functions
		for function in functions {
			let src = self.path()?.join("functions").join(function);
			let dst = base.join(function);
			// Note: configfs has some interesting behaviour with relative paths, cwd is taken into
			// account. So let configfs itself figure out the path it wants for the function.
			std::os::unix::fs::symlink(src, dst)?;
		}

		return Ok(config);
	}

	pub fn attach(&self, udc : &str) -> io::Result<()>
	{
		std::fs::write(self.path()?.join("UDC"), udc)?;
		return Ok(());
	}

	pub fn attached(&self) -> io::Result<Option<String>>
	{
		let udc = std::fs::read_to_string(self.path()?.join("UDC"))?.trim_end().to_owned();
		if udc.len() != 0 {
			return Ok(Some(udc));
		}
		return Ok(None);
	}

	pub fn detach(&self) -> io::Result<()>
	{
		return self.attach("");
	}

	fn remove_config(&self, name : &str) -> io::Result<()>
	{
		let config = self.path()?.join("configs").join(name);
		if let Ok(entries) = std::fs::read_dir(&config) {
			for i in entries {
				if let Ok(entry) = i {
					// check if symlink, aka function
					if let Ok(filetype) = entry.file_type() {
						if filetype.is_symlink() {
							// remove function
							std::fs::remove_file(entry.path())?;
						}
					}
				}
			}
		}

		// remove strings
		std::fs::remove_dir(config.join("strings").join("0x409"))?;

		// remove config
		std::fs::remove_dir(&config)?;
		return Ok(());
	}

	pub fn remove_function(&self, name : &str) -> io::Result<()>
	{
		// remove function
		let function = self.path()?.join("functions").join(name);
		std::fs::remove_dir(function)?;
		return Ok(());
	}

	pub fn configs(&self) -> io::Result<impl std::iter::Iterator<Item = String>>
	{
		return read_dir_child_dirnames(self.path()?.join("configs"));
	}

	pub fn functions(&self) -> io::Result<impl std::iter::Iterator<Item = String>>
	{
		return read_dir_child_dirnames(self.path()?.join("functions"));
	}

	pub fn cleanup(&self) -> io::Result<()>
	{
		// ensure it is not attached to a device
		self.detach()?;

		// remove configs
		for config in self.configs()? {
			self.remove_config(&config)?;
		}

		// remove functions
		for function in self.functions()? {
			self.remove_function(&function)?;
		}

		// remove strings
		std::fs::remove_dir(self.path()?.join("strings").join("0x409"))?;

		// remove gadget device
		std::fs::remove_dir(self.path()?)?;

		return Ok(());
	}

	pub fn interfaces() -> io::Result<impl std::iter::Iterator<Item = String>>
	{
		let root = Path::new("/sys/class/udc");
		let entries = std::fs::read_dir(root)?;
		return Ok(entries
				.filter_map(|e : io::Result<std::fs::DirEntry>| {
						if let Ok(entry) = e {
							let filename = entry.file_name();
							if let Some(name) = filename.to_str() {
								return Some(name.to_owned());
							}
						}
						return None;
					})
			);
	}

	pub fn first_interface() -> Option<String>
	{
		if let Ok(interfaces) = &mut Gadget::interfaces() {
			if let Some(first) = interfaces.next() {
				return Some(first);
			}
		}
		return None
	}

	pub fn has_interfaces() -> bool
	{
		if let Ok(interfaces) = Gadget::interfaces() {
			if interfaces.count() != 0 {
				return true;
			}
		}
		return false;
	}

	pub fn debug(&self) -> io::Result<()>
	{
		println!("Gadget '{}'", self.name);
		if let Some(udc) = self.attached()? {
			println!("  UDC: {}", udc);
		} else {
			println!("  UDC: (not attached)");
		}
		println!("  Functions:");
		for function in self.functions()? {
			println!("    * {}", function);
		}
		println!("  Configs:");
		for config in self.configs()? {
			println!("    * {}", config);
		}
		println!("  Configfs Tree:");
		Self::debug_configfs_tree(self.path()?.as_path(), self.path()?.as_path())?;
		return Ok(());
	}

	fn debug_configfs_tree(path : &Path, root : &Path) -> io::Result<()>
	{
		let absroot = root.canonicalize()?;
		let absbase = path.canonicalize()?;
		let relbase = absbase.strip_prefix(&absroot).unwrap_or(&absbase);

		for d in std::fs::read_dir(path).unwrap() {
			let entry = d?;

			let relpath = relbase.join(entry.file_name());
			if let Ok(filetype) = entry.file_type() {
				if filetype.is_dir() {
					Self::debug_configfs_tree(entry.path().as_path(), root)?;
					println!(" -> {}/", relpath.display());
				} else if filetype.is_file() {
					println!(" -> {}", relpath.display());
				} else if filetype.is_symlink() {
					if let Ok(linkpath) = entry.path().read_link() {
						println!(" -> {} -> {:?}", relpath.display(), linkpath);
					} else {
						println!(" -> {} -> <error>", relpath.display());
					}
				}
			}
		}
		return Ok(());
	}

	pub fn debug_interfaces() -> io::Result<()>
	{
		println!("UDCs:");
		for udc in Gadget::interfaces()? {
			println!("  * '{}'", udc);
		}
		return Ok(());
	}
}

impl Drop for Gadget {
	fn drop(&mut self) {
		self.cleanup().unwrap_or(());
	}
}
