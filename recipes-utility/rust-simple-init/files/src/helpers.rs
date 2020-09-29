use std::io;
use std::net::Ipv4Addr;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::process::Command;

pub fn start_dhcpd(iface : &str, start : Ipv4Addr, end : Ipv4Addr) -> io::Result<process::Child>
{
	// create config
	let configpath = format!("/var/run/dhcp.{}.conf", iface);
	std::fs::write(&configpath, [
		&format!("start {}", start),
		&format!("end {}", end),
		&format!("interface {}", iface),
		&format!("lease_file /var/run/dhcp.{}.leases", iface),
		"option subnet 255.255.255.252",
		"option lease 3600",
		].join("\n"))?;

	// start daemon
	let result = Command::new("/usr/sbin/udhcpd").arg(configpath).spawn();
	match &result {
		Ok(child) => { println!("init: dhcpd on {} (pid = {})", iface, child.id()); }
		Err(err) => { println!("init: failed to start dhpcd -> {}", err); }
	}
	return result;
}

pub fn has_valid_usb_device_class() -> bool
{
	let udcdir = Path::new("/sys/class/udc");
	if !udcdir.exists() || !udcdir.is_dir() {
		return false;
	}

	// check their is a device
	if let Ok(entries) = std::fs::read_dir(udcdir) {
		for entry in entries {
			if let Ok(_) = entry {
				return true; // only return true if the entry is also valid
			}
		}
	}
	return false;
}

pub fn wait_for_net_device(iface : &str)
{
	let ifacedir = Path::new("/sys/class/net").join(iface);
	loop {
		if ifacedir.exists() {
			return;
		}
		// wait 250ms
		std::thread::sleep(std::time::Duration::from_millis(250));
	}
}

pub fn shell()
{
	println!("init: shell");
	let result = Command::new("/bin/sh").status().expect("init: shell failed to start");
	match result.code()
	{
		Some(code) => {
			println!("init: shell failed with exit code {}", code);
			process::exit(code);
		}
		None => {
			println!("init: shell terminated by signal {}", result.signal().unwrap() as u8);
			process::exit(((result.signal().unwrap() as u8) + 128u8) as i32);
		}
	}
}

pub fn openssh() -> io::Result<process::Child>
{
	// check keys
	let keydir = "/etc/ssh";
	std::fs::create_dir_all(keydir)?;
	for key in ["rsa", "ecdsa", "ed25519"].iter()
	{
		let keyfile : PathBuf = [keydir, &format!("ssh_host_{}_key", key)].iter().collect();
		if !keyfile.exists() {
			println!("init: sshd - generating {}", key);
			Command::new("ssh-keygen").arg("-q")
				.arg("-f").arg(keyfile)
				.arg("-N").arg("")
				.arg("-t").arg(key).status()?;
		}
	}

	println!("init: sshd");
	std::fs::create_dir_all("/var/run/sshd")?;
	let result = Command::new("/usr/sbin/sshd").spawn();
	if let Ok(child) = &result {
		println!("init: sshd (pid = {})", child.id());
	} else if let Err(err) = &result {
		println!("init: failed to start sshd -> {}", err);
	}
	return result;
}

