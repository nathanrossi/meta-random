use std::io;
use std::path::Path;
use std::process::Command;
use std::net::Ipv4Addr;

mod lib;
use lib::configfs;
use lib::runtime::{Runtime, ServiceManager};
use lib::service::ProcessService;
use lib::console::ConsoleService;
use lib::network;
use lib::network::NetworkDeviceService;
use lib::openssh::SSHService;

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>>
{
	println!("[init] started");

	println!("[init] setting hostname");
	if let Err(_) = nix::unistd::sethostname("rpi") {
		println!("[init] failed to set hostname");
	}

	let mut manager = ServiceManager::new();
	let mut rt = Runtime::new().unwrap();

	{
		let mut service = lib::mount::MountSetup::new();
		// procfs is needed first in order to check mounts
		service.add("proc", Some("proc"), "/proc", None);
		service.add("sysfs", Some("sysfs"), "/sys", None);
		// device nodes
		service.add("devtmpfs", None, "/dev", Some("mode=0755"));
		// /dev/pts and /dev/ptmx
		service.add("devpts", Some("devpts"), "/dev/pts", Some("mode=0620,ptmxmode=0666,gid=5"));
		// setup later mounts
		service.add("tmpfs", Some("tmpfs"), "/run", Some("mode=0755,nodev,nosuid,strictatime"));
		service.add("tmpfs", Some("tmpfs"), "/var/volatile", None);
		// kernel debug
		service.add("debugfs", None, "/sys/kernel/debug", None);

		let instance = manager.add_service(&rt, service, true);
		rt.poll_service_ready(&mut manager, &instance)?; // wait for service to complete
	}

	// add serial consoles
	manager.add_service(&rt, ConsoleService::new("ttyACM0", 115200, true), true);
	manager.add_service(&rt, ConsoleService::new("ttyAMA0", 115200, true), true); // qemuarm serial
	// manager.add_service(&rt, ConsoleService::new("ttyUSB0", 115200, true), true);

	// modprobe camera driver
	let modprobe = manager.add_service(&rt, ProcessService::oneshot("/sbin/modprobe", &["bcm2835-v4l2"]), true);
	rt.poll_service_ready(&mut manager, &modprobe)?;

	println!("[init] usb device class");
	// configfs::usb::Gadget::debug_interfaces();

	// add usb gadget
	let firstudc = configfs::usb::Gadget::first_interface();
	if let Some(udc) = firstudc {
		manager.add_service(&rt, ConsoleService::new("ttyGS0", 115200, true), true); // gadget serial
		manager.add_service(&rt, NetworkDeviceService::new("usb0", network::Config::StaticIpv4(Ipv4Addr::new(169, 254, 1, 1), 30, None)), true);
		// start_dhcpd("usb0", Ipv4Addr::new(169, 254, 1, 2), Ipv4Addr::new(169, 254, 1, 2));
		manager.add_service(&rt, lib::service::UsbGadgetService::new(&udc, || {
				// setup usb0 (usb gadget)
				let device = configfs::usb::Gadget::create("0", "Nathan Rossi", "Pi Zero Camera")?;

				let devicepath = device.path()?;
				std::fs::write(&devicepath.join("bcdDevice"), "0x0100\n")?;
				std::fs::write(&devicepath.join("bcdUSB"), "0x0100\n")?;
				std::fs::write(&devicepath.join("bDeviceClass"), "0xef\n")?;
				std::fs::write(&devicepath.join("bDeviceSubClass"), "0x02\n")?;
				std::fs::write(&devicepath.join("bDeviceProtocol"), "0x01\n")?;

				let serial = device.add_function("acm", "GS0")?;
				// let network = device.add_function("eem", "usb0")?;

				// uvc
				println!("[usbgadget] setting up uvc");
				let uvc = device.add_function("uvc", "0");
				if let Err(e) = &uvc {
					println!("[usbgadget] error = {}", e);
					device.debug()?;
					return Ok(None);
				}
				let uvc = uvc.unwrap();
				println!("[usbgadget] got uvc '{}'", uvc);

				let functionpath = device.path()?.join("functions").join(&uvc);

				// create frame info
				let streampath = functionpath.join("streaming").join("mjpeg").join("m").join("720p");
				println!("[usbgadget] creating streampath");
				std::fs::create_dir_all(&streampath)?;
				println!("[usbgadget] writing frame data");
				std::fs::write(&streampath.join("wWidth"), "1280\n")?;
				std::fs::write(&streampath.join("wHeight"), "720\n")?;
				std::fs::write(&streampath.join("dwMinBitRate"), "10000000\n")?;
				std::fs::write(&streampath.join("dwMaxBitRate"), "100000000\n")?;
				std::fs::write(&streampath.join("dwMaxVideoFrameBufferSize"), "1843200\n")?; // W * H * 2
				// std::fs::write(&streampath.join("dwFrameInterval"), "666666\n100000\n5000000\n")?; // (n * 100) ns -> fps = 1 / (v * 10 000 000)
				std::fs::write(&streampath.join("dwFrameInterval"), "5000000\n")?; // (n * 100) ns -> fps = 1 / (v * 10 000 000)

				// setup header info symlinks
				let headerpath = functionpath.join("streaming").join("header").join("h");
				println!("[usbgadget] create header info");
				std::fs::create_dir_all(&headerpath)?;
				let streamroot = functionpath.join("streaming").join("mjpeg").join("m");
				println!("[usbgadget] linking stream info to header info");
				std::os::unix::fs::symlink(&streamroot, headerpath.join("m"))?;

				// setup class header links
				let classpath = functionpath.join("streaming").join("class").join("fs");
				println!("[usbgadget] create classpath {:?}", &classpath);
				// std::fs::create_dir_all(&classpath)?;
				println!("[usbgadget] linking header info to fs class info");
				std::os::unix::fs::symlink(&headerpath, classpath.join("h"))?;
				let classpath = functionpath.join("streaming").join("class").join("hs");
				println!("[usbgadget] create classpath {:?}", &classpath);
				// std::fs::create_dir_all(&classpath)?;
				println!("[usbgadget] linking header info to hs class info");
				std::os::unix::fs::symlink(&headerpath, classpath.join("h"))?;

				// setup control class links
				let controlpath = functionpath.join("control").join("header").join("h");
				let classpath = functionpath.join("control").join("class");
				println!("[usbgadget] create control path {:?}", &controlpath);
				std::fs::create_dir_all(&controlpath)?;

				// device.debug();
				println!("[usbgadget] linking control header info to class");
				std::os::unix::fs::symlink(&controlpath, classpath.join("fs").join("h"))?; // class/fs/h -> header/h
				// std::os::unix::fs::symlink(&controlpath, classpath.join("hs").join("h"))?; // class/hs/h -> header/h, does not exist?
				// std::os::unix::fs::symlink(&controlpath, classpath.join("ss").join("h"))?;

				// set packet size (2K)
				println!("[usbgadget] packet size");
				std::fs::write(functionpath.join("streaming_maxpacket"), "2048")?;

				println!("[usbgadget] device config?");
				// device.add_config("Serial, Networking & UVC", &[&serial, &network, &uvc])?;
				let config = device.add_config("Serial, Networking & UVC", &[&serial, &uvc])?;
				let configpath = device.path()?.join("configs").join(config);
				std::fs::write(&configpath.join("MaxPower"), "500\n")?;

				// device.add_config("Serial, Networking & UVC", &[&uvc])?;
				return Ok(Some(device));
			}), true);
	}

	// NOTE: https://forums.developer.nvidia.com/t/jetson-tk1-behaving-like-an-usb-camera/40316/15
	// NOTE: http://www.davidhunt.ie/raspberry-pi-zero-with-pi-camera-as-usb-webcam/

	// network devices
	manager.add_service(&rt, NetworkDeviceService::new("eth0", network::Config::DHCP), true);

	// manager.add_service(&rt, ProcessService::new("/usr/sbin/rngd", &["-r", "/dev/hwrng"]), true);
	// manager.add_service(&rt, SSHService::default(), true);

	// manager.add_service(&rt, ProcessService::new("/usr/bin/uvc-gadget", &["-i", "/usr/share/sample.jpg", "uvc.0"]), true);
	manager.add_service(&rt, ProcessService::new("/usr/bin/uvc-gadget", &["-c", "/dev/video0", "uvc.0"]), true);

	// manager.add_service(&rt, ProcessService::new("/usr/bin/uvc-gadget",
		// &["-d", "-f", "mjpeg", "-i", "/usr/share/sample.jpg", "-n3", "-r1", "-s1", "-u", "/dev/video0"]), true);
	// manager.add_service(&rt, ProcessService::new("/usr/bin/uvc-gadget",
		// &["-d", "-f", "1", "-n3", "-r1", "-s1", "-u", "/dev/video0"]), true);
	// manager.add_service(&rt, ProcessService::new("/usr/bin/uvc-gadget",
		// &["-d", "-f", "1", "-n3", "-r1", "-s1", "-v", "/dev/video0", "-u", "/dev/video1"]), true);
	// manager.add_service(&rt, ProcessService::new("/usr/bin/uvc-gadget",
		// &["-f", "1", "-r", "1", "-v", "/dev/video0", "-u", "/dev/video1"]), true);

	return rt.poll(&mut manager);
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

