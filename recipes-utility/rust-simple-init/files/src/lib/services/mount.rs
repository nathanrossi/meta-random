use std::process::{Child};
use super::super::*;
use procfs;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;

enum State
{
	None,
	Start,
	Waiting(Option<Child>, usize),
	Complete,
	Error,
}

pub struct MountEntry
{
	fstype : String,
	device : Option<String>,
	point : String,
	options : Option<String>,
}

pub struct MountSetup
{
	mounts : Vec<MountEntry>,
	state : State,
}

impl MountSetup
{
	pub fn new() -> Self
	{
		return Self { mounts : Vec::new(), state : State::None };
	}

	pub fn add(&mut self, fstype : &str, device : Option<&str>, point : &str, options : Option<&str>)
	{
		self.mounts.push(MountEntry {
			fstype : fstype.to_owned(),
			device : device.map(|s| s.to_owned()),
			point : point.to_owned(),
			options : options.map(|s| s.to_owned()),
			});
	}

	fn mount(&self, runtime : &mut Runtime, mount : &MountEntry) -> std::io::Result<Option<Child>>
	{
		// check if already mounted
		if procfs::mounted(&mount.point, mount.device.as_deref(), Some(&mount.fstype)) {
			runtime.logger.service_log("mount", &format!("{} skipping, already mounted", mount.point));
			return Ok(None);
		}

		// check/make the mount point
		std::fs::create_dir_all(&mount.point)?;

		let mut command = std::process::Command::new("/bin/mount");
		command.args(&["-t", &mount.fstype]);

		// device argument
		if let Some(d) = &mount.device {
			command.arg(d);
		} else {
			command.arg("none");
		}

		command.arg(&mount.point);

		// options arguments
		if let Some(o) = &mount.options {
			command.arg("-o");
			command.arg(o);
		}

		let child = command.spawn()?;
		runtime.logger.service_log("mount", &format!("{} mounting", mount.point));
		return Ok(Some(child));
	}

	fn check(&mut self, runtime : &mut Runtime)
	{
		loop {
			let index = match self.state {
					State::Start => { 0 }
					State::Waiting(_, i) => { i + 1 }
					_ => { return; }
				};

			if index >= self.mounts.len() {
				self.state = State::Complete;
				return;
			}

			let entry = &self.mounts[index];
			if let Ok(c) = self.mount(runtime, &entry) {
				if let Some(child) = c {
					self.state = State::Waiting(Some(child), index);
				} else {
					self.state = State::Waiting(None, index);
					continue;
				}
			} else {
				self.state = State::Error;
			}
			return;
		}
	}
}

impl Service for MountSetup
{
	fn setup(&mut self, _runtime : &mut Runtime)
	{
	}

	fn state(&self) -> ServiceState
	{
		if let State::Complete = self.state {
			return ServiceState::Completed;
		} else if let State::Waiting(_, _) = self.state {
			return ServiceState::Starting;
		} else if let State::Error = self.state {
			return ServiceState::Error;
		}
		return ServiceState::Inactive;
	}

	fn start(&mut self, runtime : &mut Runtime)
	{
		if let State::None = self.state {
			if self.mounts.len() > 0 {
				self.state = State::Start;
				self.check(runtime);
			}
		}
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
		// TODO: unmount?
		// self.state = State::None;
	}

	fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
	{
		if let ServiceEvent::ProcessExited(pid, status) = event {
			if let State::Waiting(child, index) = &self.state {
				if let Some(child) = child {
					if child.id() != pid {
						return false;
					}
				}
				if status.success() {
					self.check(runtime);
				} else {
					runtime.logger.service_log("mount", &format!("{}, failed to mount (err = {})", self.mounts[*index].point, status));
					self.state = State::Error; // TODO: allow some mounts to fail?
				}
				return true;
			}
		}
		return false;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use service::ServiceManager;

	#[test]
	fn configure()
	{
		let mut manager = ServiceManager::new();
		let mut rt = Runtime::new_default_logger().unwrap();
		let mut service = MountSetup::new();

		// device nodes
		service.add("devtmpfs", None, "/dev", Some("mode=0755"));
		// /dev/pts and /dev/ptmx
		service.add("devpts", Some("devpts"), "/dev/pts", Some("mode=0620,ptmxmode=0666,gid=5"));
		// setup later mounts
		service.add("proc", Some("proc"), "/proc", None);
		service.add("sysfs", Some("sysfs"), "/sys", None);
		service.add("tmpfs", Some("tmpfs"), "/run", Some("mode=0755,nodev,nosuid,strictatime"));
		service.add("tmpfs", Some("tmpfs"), "/var/volatile", None);
		// kernel debug
		service.add("debugfs", None, "/sys/kernel/debug", None);

		manager.add_service(&mut rt, service,  false);

		// TODO: cannot really do emulation of devices for integration testing
		// rt.poll(&mut manager);
	}
}
