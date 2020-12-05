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
		// mount /boot
		service.add("auto", Some("/dev/mmcblk0p1"), "/boot", Some("ro"));

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
	// modprobe wifi
	let modprobe = manager.add_service(&rt, ProcessService::oneshot("/sbin/modprobe", &["brcmfmac"]), true);
	rt.poll_service_ready(&mut manager, &modprobe)?;

	println!("[init] usb device class");
	// configfs::usb::Gadget::debug_interfaces();

	// add usb gadget
	let firstudc = configfs::usb::Gadget::first_interface();
	if let Some(udc) = firstudc {
		manager.add_service(&rt, ConsoleService::new("ttyGS0", 115200, true), true); // gadget serial
		let mut usb0 = NetworkDeviceService::new("usb0");
		usb0.add(network::Config::StaticIpv4(Ipv4Addr::new(169, 254, 1, 1), 30, None));
		// start_dhcpd("usb0", Ipv4Addr::new(169, 254, 1, 2), Ipv4Addr::new(169, 254, 1, 2));
		usb0.add(network::Config::DHCPD(Ipv4Addr::new(169, 254, 1, 2), Ipv4Addr::new(169, 254, 1, 2)));
		manager.add_service(&rt, usb0, true);
		manager.add_service(&rt, lib::service::UsbGadgetService::new(&udc, || {
				// setup usb0 (usb gadget)
				let device = configfs::usb::Gadget::create("0", "Nathan Rossi", "Pi Zero")?;

				let serial = device.add_function("acm", "GS0")?;
				let network = device.add_function("eem", "usb0")?;

				let config = device.add_config("Serial + Networking", &[&serial, &network])?;
				let configpath = device.path()?.join("configs").join(config);
				std::fs::write(&configpath.join("MaxPower"), "500\n")?;
				return Ok(Some(device));
			}), true);
	}

	// network devices
	let mut eth0 = NetworkDeviceService::new("eth0");
	eth0.add(network::Config::DHCP);
	manager.add_service(&rt, eth0, true);

	let mut wlan0 = NetworkDeviceService::new("wlan0");
	wlan0.add(network::Config::WPASupplicant("/boot/wifi.conf".to_owned()));
	wlan0.add(network::Config::DHCP);
	manager.add_service(&rt, wlan0, true);

	// services
	manager.add_service(&rt, ProcessService::new("/usr/sbin/rngd", &["-r", "/dev/hwrng"]), true);
	manager.add_service(&rt, SSHService::default(), true);

	// mjpeg streaming of camera
	let mut mjpg = ProcessService::new("/usr/bin/mjpg_streamer", &[
			"-i", "/usr/lib/mjpg-streamer/input_raspicam.so -x 1296 -y 972 -fps 15 -ISO 50 -quality 90",
			"-i", "/usr/lib/mjpg-streamer/input_uvc.so -resolution 1296x972 -fps 15 -d /dev/video0",
			"-o", "/usr/lib/mjpg-streamer/output_http.so -p 80"]);
	mjpg.add_device_dependency("/dev/video0"); // don't start until /dev/video0 is available
	manager.add_service(&rt, mjpg, true);

	return rt.poll(&mut manager);
}

