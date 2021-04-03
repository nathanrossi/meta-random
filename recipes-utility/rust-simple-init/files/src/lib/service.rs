use std::rc::Rc;
use std::cell::RefCell;
use std::os::unix::io::RawFd;
use nix::unistd::Pid;
use super::runtime::Runtime;
use super::uevent;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ServiceState
{
	Unknown,
	Inactive,
	Starting,
	Running,
	Completed,
	Error,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ServiceEvent<'a>
{
	ProcessExited(u32, std::process::ExitStatus),
	Fd(std::os::unix::io::RawFd),
	Device(&'a uevent::EventData),
	Invalid,
}

pub trait Service
{
	fn setup(&mut self, runtime : &mut Runtime);
	fn state(&self) -> ServiceState;
	fn start(&mut self, runtime : &mut Runtime);
	fn stop(&mut self, runtime : &mut Runtime);
	fn event<'a>(&mut self, runtime : &mut Runtime, event : ServiceEvent<'a>) -> bool;
}

pub type ServiceRef<'a> = Rc<RefCell<Box<dyn Service + 'a>>>;

pub struct ServiceManager<'a>
{
	services : Vec<ServiceRef<'a>>,
}

impl<'a> ServiceManager<'a>
{
	pub fn new() -> Self
	{
		return Self { services : Vec::new() };
	}

	pub fn add_service(&mut self, runtime : &mut Runtime, service : impl Service + 'a, enabled : bool) -> ServiceRef<'a>
	{
		let instance : ServiceRef<'a> = Rc::new(RefCell::new(Box::new(service)));
		instance.borrow_mut().setup(runtime);
		if enabled {
			instance.borrow_mut().start(runtime);
		}
		self.services.push(instance.clone());
		return instance;
	}

	pub fn process_event(&mut self, runtime : &mut Runtime, pid : Pid, status : std::process::ExitStatus) -> bool
	{
		// convert Pid to .id() equivalent for child process
		let id = libc::pid_t::from(pid) as u32;
		for service in self.services.iter_mut() {
			if service.borrow_mut().event(runtime, ServiceEvent::ProcessExited(id, status)) {
				return true;
			}
		}
		return false;
	}

	pub fn fd_event(&mut self, runtime : &mut Runtime, fd : RawFd) -> bool
	{
		for service in self.services.iter_mut() {
			if service.borrow_mut().event(runtime, ServiceEvent::Fd(fd)) {
				return true;
			}
		}
		return false;
	}

	pub fn device_event(&mut self, runtime : &mut Runtime, event : &uevent::EventData) -> bool
	{
		let mut handled = false;
		for service in self.services.iter_mut() {
			if service.borrow_mut().event(runtime, ServiceEvent::Device(event)) {
				handled = true;
			}
		}
		return handled;
	}
}

