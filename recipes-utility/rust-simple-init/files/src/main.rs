use std::io;
use std::path::Path;
use std::process::Command;
// use tokio::prelude::*;
use std::sync::Arc;

mod lib;
use lib::configfs;
use lib::uevent;
mod helpers;
use helpers::shell;

fn setup_usb_gadgets() -> io::Result<Option<configfs::usb::Gadget>>
{
	// select first udc
	let interfaces = configfs::usb::Gadget::interfaces()?.next();
	if let Some(first) = interfaces {
		// setup usb0 (usb gadget)
		println!("init: setup usb gadget");
		let device = configfs::usb::Gadget::create("0", "Nathan Rossi", "Pi Zero Camera")?;
		let serial = device.add_function("acm", "GS0")?;
		let network = device.add_function("eem", "usb0")?;
		let config = device.add_config("Serial & Networking", &[&serial, &network])?;

		// attach to port
		println!("init: usb gadget attaching to {}", first);
		device.attach(&first)?;

		return Ok(Some(device));
	}
	return Ok(None);
}

#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error>>
{
	println!("init: started");

	//println!("init: set hostname");
	//nix::unistd::sethostname("rpi");

	// setup_early_mounts();

	// setup loopback
	// Command::new("/sbin/ip").args(&["link", "set", "dev", "lo", "up"]).status();

	/*
	// start rngd for entropy
	println!("init: starting rngd");
	Command::new("/usr/sbin/rngd").arg("-f").arg("-r").arg("/dev/hwrng").spawn();

	println!("init: setup usb gadget?");
	configfs::usb::Gadget::debug_interfaces();

	// setup usb gadget if available
	let getty;
	let gadget : Option<configfs::usb::Gadget>;
	if let Ok(device) = setup_usb_gadgets() {
		gadget = device;

		// TODO: networking setup
		// Command::new("/sbin/ip").args(&["link", "set", "dev", "usb0", "up"]).status();
		// Command::new("/sbin/ip").args(&["addr", "add", "169.254.1.1/30", "dev", "usb0"]).status();
		// start_dhcpd("usb0", Ipv4Addr::new(169, 254, 1, 2), Ipv4Addr::new(169, 254, 1, 2));

		getty = Command::new("/sbin/getty").args(&["-i", "-w", "-L", "115200", "/dev/ttyGS0"]).spawn();
	}
	*/

	let mut poll = mio::Poll::new()?;
	let mut events = mio::Events::with_capacity(64);

	let mut consoles = lib::consoles::Manager::new();
	consoles.add("ttyACM0", 115200)?;

	let mut s = uevent::Socket::open().unwrap();
	let mut sm = uevent::DeviceMonitor::new();
	sm.register_subsystem("tty", |_| { println!("got event for tty"); });
	// sm.register_subsystem("tty", |e| { consoles.check_uevent(e); });

	poll.register(&mut s, mio::Token(0), mio::Ready::readable(), mio::PollOpt::edge())?;

	tokio::spawn(async move {
			let mut s2 = uevent::SocketAsync::open().unwrap();
			println!("init: uevent monitor starting");
			loop {
				if let Ok(e) = s2.read().await {
					if let Some(event) = e {
						println!("got event");
						sm.process_event(&event);
					}
				}
			}
		}).await?;

	// loop {
		// poll.poll(&mut events, None)?;
		// for event in &events {
			// if event.token() == mio::Token(0) {
				// sm.recv(&s)?;
			// }
		// }
	// }

	/*
	if let Some(action) = event.get("ACTION") {
		if action == "add" || action == "remove" {
			if let Some(subsys) = event.get("SUBSYSTEM") {
				} if subsys == "net" {
					if let Some(iface) = event.get("ID_NET_NAME_PATH") {
						println!("change to network iface '{}'", iface);
					}
				}
			}
		}
	}
	*/

	// wait for eth0 to appear, on some boards it can be "slow" due to USB
	// println!("init: waiting for eth0");
	// wait_for_net_device("eth0");
	// println!("init: setup eth0");
	// Command::new("/sbin/ip").args(&["link", "set", "dev", "eth0", "up"]).status();
	// Command::new("/sbin/udhcpc").args(&["--interface=eth0"]).status();

	// start ssh
	// openssh();

	// setup_rtsp_camera();

	// shell();

	return Ok(());
}

fn setup_rtsp_camera() -> io::Result<()>
{
	println!("init: load raspberry pi v4l2 driver");
	Command::new("/sbin/modprobe").arg("bcm2835-v4l2").status()?;

	let videonode = Path::new("/dev/video0");
	if !videonode.exists() {
		println!("init: no video device, skipping rtsp/camera setup");
		return Ok(());
	}

	println!("init: starting rtsp server");
	let rtsp = Command::new("/usr/bin/python3").arg("/usr/bin/rtsp-restreamer").spawn();
	if let Ok(child) = rtsp {
		println!("init: rtsp (pid = {})", child.id());
	} else if let Err(err) = rtsp {
		println!("init: failed to start rtsp -> {}", err);
		return Err(err);
	}

	println!("init: starting raspberry pi rtp camera stream");
	Command::new("ffmpeg")
		.arg("-f").arg("video4linux2")
		.arg("-input_format").arg("h264")
		.arg("-video_size").arg("1280x960")
		.arg("-framerate").arg("30")
		.arg("-i").arg("/dev/video0")
		.arg("-vcodec").arg("copy")
		.arg("-an")
		.arg("-f").arg("rtsp")
		.arg("-rtsp_transport").arg("tcp")
		.arg("rtsp://127.0.0.1/").status()?;

	return Ok(());
}

fn setup_early_mounts() -> io::Result<()>
{
	println!("init: early mounts");
	// mount /dev
	// TODO: need to check if already mounted
	Command::new("/bin/mount").args(&["-t", "devtmpfs", "none", "/dev", "-o", "mode=0755"]).status()?;
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

