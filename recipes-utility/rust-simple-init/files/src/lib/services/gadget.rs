use std::io;
use super::super::*;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;
use configfs::usb::Gadget;

pub struct UsbGadgetService<'a>
{
	name : String,
	configfn : Box<dyn Fn() -> io::Result<Option<Gadget>> + 'a>,
	device : Option<Gadget>,
	failed : bool,
}

impl<'a> UsbGadgetService<'a>
{
	pub fn new(device : &str, config : impl Fn() -> io::Result<Option<Gadget>> + 'a) -> Self
	{
		return Self { name : device.to_owned(), configfn : Box::new(config), device : None, failed : false };
	}

	pub fn available(&self) -> bool
	{
		if let Ok(interfaces) = configfs::usb::Gadget::interfaces() {
			for interface in interfaces {
				if interface == self.name {
					return true;
				}
			}
		}
		return false;
	}
}

impl<'a> Service for UsbGadgetService<'a>
{
	fn setup(&mut self, _runtime : &mut Runtime)
	{
	}

	fn state(&self) -> ServiceState
	{
		if self.failed {
			return ServiceState::Error;
		}
		if let Some(_) = self.device {
			return ServiceState::Completed;
		}
		return ServiceState::Inactive;
	}

	fn start(&mut self, runtime : &mut Runtime)
	{
		if self.failed {
			return;
		}

		if !self.available() {
			return
		}

		if let None = self.device {
			runtime.logger.service_log("usbgadget", "setting up device");
			let result = (&self.configfn)();
			if let Ok(d) = result {
				if let Some(device) = d {
					runtime.logger.service_log("usbgadget", "device configured");

					// attach to port
					runtime.logger.service_log("usbgadget", &format!("usb gadget attaching to {}", self.name));
					if let Ok(_) = device.attach(&self.name) {
						self.device = Some(device);
					}
				}
			} else if let Err(e) = result {
				runtime.logger.service_log("usbgadget", &format!("setting up device failed! (err = {})", e));
				self.failed = true;
			}
		}
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
	}

	fn event(&mut self, _runtime : &mut Runtime, _event : ServiceEvent) -> bool
	{
		return false;
	}
}
