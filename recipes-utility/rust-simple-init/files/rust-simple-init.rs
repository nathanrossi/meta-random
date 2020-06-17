//use nix::unistd;
//use std::ffi::OsStr;
use std::io;
use std::net::Ipv4Addr;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::process::Command;

fn start_dhcpd(iface : &str, start : Ipv4Addr, end : Ipv4Addr) -> io::Result<process::Child>
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

fn has_valid_usb_device_class() -> bool
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

fn main()
{
	println!("init: started");

	//println!("init: set hostname");
	//nix::unistd::sethostname("rpi");

	setup_early_mounts();

	// start rngd for entropy
	println!("init: starting rngd");
	Command::new("/usr/sbin/rngd").arg("-f").arg("-r").arg("/dev/hwrng").spawn();

	// check if any usb gadget capable ports exist
	if has_valid_usb_device_class() {
		// setup usb0 (usb gadget)
		println!("init: setup usb gadget");
		Command::new("/sbin/modprobe").arg("g_ether").status();
		Command::new("/sbin/ip").args(&["link", "set", "dev", "usb0", "up"]).status();
		Command::new("/sbin/ip").args(&["addr", "add", "169.254.1.1/30", "dev", "usb0"]).status();
		start_dhcpd("usb0", Ipv4Addr::new(169, 254, 1, 2), Ipv4Addr::new(169, 254, 1, 2));
	}

	// start ssh
	openssh();

	println!("init: load raspberry pi v4l2 driver");
	Command::new("/sbin/modprobe").arg("bcm2835-v4l2").status();

	println!("init: starting raspberry pi rtp camera stream");
	Command::new("ffmpeg")
		.arg("-f").arg("video4linux2")
		.arg("-input_format").arg("h264")
		.arg("-video_size").arg("1280x960")
		.arg("-framerate").arg("30")
		.arg("-i").arg("/dev/video0")
		.arg("-vcodec").arg("copy")
		.arg("-an")
		.arg("-f").arg("mpegts")
		.arg("udp://169.254.1.2:8000").status();

	shell();
}

fn setup_early_mounts() -> io::Result<()>
{
	println!("init: early mounts");
	// mount /dev
	// TODO: need to check if already mounted
	Command::new("/bin/mount").args(&["-t", "devtmpfs", "none", "/dev", "-o", "mode=0755"]).status();
	// /dev/pts and /dev/ptmx
	std::fs::create_dir_all("/dev/pts")?;
	Command::new("/bin/mount").args(&["-t", "devpts", "devpts", "/dev/pts", "-o", "mode=0620,ptmxmode=0666,gid=5"]).status()?;
	// setup later mounts
	Command::new("/bin/mount").args(&["-t", "proc", "proc", "/proc"]).status()?;
	Command::new("/bin/mount").args(&["-t", "sysfs", "sysfs", "/sys"]).status()?;
	Command::new("/bin/mount").args(&["-t", "tmpfs", "tmpfs", "/run", "-o", "mode=0755,nodev,nosuid,strictatime"]).status()?;
	Command::new("/bin/mount").args(&["-t", "tmpfs", "tmpfs", "/var/volatile"]).status()?;
	// kernel debug
	Command::new("/bin/mount").args(&["-t", "debugfs", "none", "/sys/kernel/debug"]).status()?;

	return Ok(());
}

fn shell()
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

fn openssh() -> io::Result<process::Child>
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

