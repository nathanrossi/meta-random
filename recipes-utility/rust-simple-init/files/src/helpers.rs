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

