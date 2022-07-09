use std::io;
use std::net::Ipv4Addr;

mod lib;
use lib::configfs;
use lib::runtime::Runtime;
use lib::service::ServiceManager;
use lib::services;
use services::process::ProcessService;
use services::mount;
use services::console::ConsoleService;
use services::network;
use services::network::NetworkDeviceService;
use services::dev::DeviceManagerService;
use services::openssh::SSHService;
use lib::logging::Logger;

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>>
{
	let mut logger = Logger::new();
	logger.add(io::stdout());
	logger.service_log("init", "started");

	logger.service_log("init", "setting hostname");
	if let Err(_) = nix::unistd::sethostname("rpi") {
		logger.service_log("init", "failed to set hostname");
	}

	let mut manager = ServiceManager::new();
	let mut rt = Runtime::new(logger).unwrap();

	{
		let mut service = mount::MountSetup::new();
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

		let instance = manager.add_service(&mut rt, service, true);
		rt.poll_service_ready(&mut manager, &instance)?; // wait for service to complete
	}

	{
		// handle /boot auto mount if the device exists
		if std::path::Path::new("/dev/mmcblk0p1").exists() {
			let mut service = mount::MountSetup::new();
			// mount /boot
			service.add("auto", Some("/dev/mmcblk0p1"), "/boot", Some("ro"));

			let instance = manager.add_service(&mut rt, service, true);
			rt.poll_service_ready(&mut manager, &instance)?; // wait for service to complete
		}
	}

	rt.logger.service_log("init", "initial mounts complete");

	// requires the /var/volatile mount
	rt.logger.add_file("/var/volatile/log/messages")?;
	rt.logger.service_log("init", "created log file for messages");

	// start device manager
	manager.add_service(&mut rt, DeviceManagerService::new(), true);

	// add serial consoles
	// manager.add_service(&mut rt, ConsoleService::new("ttyACM0", 115200, true), true);
	// manager.add_service(&mut rt, ConsoleService::new("ttyAMA0", 115200, true), true);
	// manager.add_service(&mut rt, ConsoleService::new("ttyUSB0", 115200, true), true);

	rt.logger.service_log("init", "usb device class");
	// configfs::usb::Gadget::debug_interfaces();

	// add usb gadget
	let firstudc = configfs::usb::Gadget::first_interface();
	if let Some(udc) = firstudc {
		manager.add_service(&mut rt, ConsoleService::new("ttyGS0", 115200, true), true); // gadget serial
		let mut usb0 = NetworkDeviceService::new("usb0");
		usb0.add(network::Config::StaticIpv4(Ipv4Addr::new(169, 254, 1, 1), 30, None));
		usb0.add(network::Config::DHCPD(Ipv4Addr::new(169, 254, 1, 2), Ipv4Addr::new(169, 254, 1, 2)));
		manager.add_service(&mut rt, usb0, true);
		manager.add_service(&mut rt, services::gadget::UsbGadgetService::new(&udc, || {
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
	let mut lo = NetworkDeviceService::new("lo");
	lo.add(network::Config::LinkUp);
	manager.add_service(&mut rt, lo, true);

	let mut eth0 = NetworkDeviceService::new("eth0");
	eth0.add(network::Config::DHCP);
	manager.add_service(&mut rt, eth0, true);

	let mut wlan0 = NetworkDeviceService::new("wlan0");
	wlan0.add(network::Config::WPASupplicant("/boot/wifi.conf".to_owned()));
	wlan0.add(network::Config::DHCP);
	manager.add_service(&mut rt, wlan0, true);

	// services
	manager.add_service(&mut rt, ProcessService::new("/usr/sbin/rngd", &["-f", "-r", "/dev/hwrng"]), true);
	manager.add_service(&mut rt, SSHService::default(), true);

	// mjpeg streaming of camera
	let mut mjpg = ProcessService::new("/usr/bin/mjpg_streamer", &[
			// "-i", "/usr/lib/mjpg-streamer/input_raspicam.so -x 1296 -y 972 -fps 15 -ISO 50 -quality 90",
			"-i", "/usr/lib/mjpg-streamer/input_uvc.so -resolution 1296x972 -fps 15 -d /dev/video0",
			"-o", "/usr/lib/mjpg-streamer/output_http.so -p 80"]);
	mjpg.add_device_dependency("/dev/video0"); // don't start until /dev/video0 is available
	manager.add_service(&mut rt, mjpg, true);

	return rt.poll(&mut manager, false);
}

