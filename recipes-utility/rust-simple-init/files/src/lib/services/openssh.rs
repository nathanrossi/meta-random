use std::path::PathBuf;
use std::os::unix::io::AsRawFd;
use std::process::Command;
use super::super::*;
use service::{Service, ServiceEvent, ServiceState};
use runtime::Runtime;

enum State
{
	None,
	KeygenRunning(usize, std::process::Child),
	Running(std::process::Child),
	Error,
}

pub struct SSHService
{
	keys : Vec<String>,
	state : State,
}

impl SSHService
{
	pub fn new<'a>(keys : &[&str]) -> Self
	{
		let k : Vec<String> = keys.iter().map(|s| s.to_string()).collect();
		return Self { keys : k, state : State::None };
	}

	pub fn default() -> Self
	{
		return Self::new(&["rsa", "ecdsa", "ed25519"]);
	}

	fn start_keygen(&mut self, runtime : &mut Runtime, index : usize) -> State
	{
		let mut next = index;
		while next < self.keys.len() {
			let key = &self.keys[next];

			let keydir = "/etc/ssh";
			if next == 0 {
				if let Err(_) = std::fs::create_dir_all(keydir) {
					runtime.logger.service_log("sshd", &format!("failed to create key directory {}", keydir));
					return State::Error;
				}
			}

			let keyfile : PathBuf = [keydir, &format!("ssh_host_{}_key", key)].iter().collect();
			if !keyfile.exists() {
				let mut command = Command::new("ssh-keygen");
				command.arg("-q")
					.arg("-f").arg(keyfile)
					.arg("-N").arg("")
					.arg("-t").arg(key);

				if let Ok(child) = command.spawn() {
					runtime.logger.service_log("sshd", &format!("generating {} key", key));
					return State::KeygenRunning(next, child);
				} else {
					runtime.logger.service_log("sshd", &format!("failed to create key {}", key));
					return State::Error;
				}
			}

			// key is already valid, skip
			next = next + 1;
		}
		return self.start_sshd(runtime);
	}

	fn start_sshd(&mut self, runtime : &mut Runtime) -> State
	{
		if let Err(_) = std::fs::create_dir_all("/var/run/sshd") {
			runtime.logger.service_log("sshd", "failed to create run directory");
			return State::Error;
		}

		if let Ok(child) = Command::new("/usr/sbin/sshd").arg("-D")
				.stdin(std::process::Stdio::null())
				.stdout(std::process::Stdio::piped())
				.stderr(std::process::Stdio::piped())
				.spawn() {
			runtime.logger.service_log("sshd", &format!("running, pid = {}", child.id()));
			if let Some(fd) = &child.stdout {
				runtime.register(fd, true, false).ok();
			}
			if let Some(fd) = &child.stderr {
				runtime.register(fd, true, false).ok();
			}
			return State::Running(child);
		}
		runtime.logger.service_log("sshd", "failed to start sshd");
		return State::Error;
	}
}

impl Service for SSHService
{
	fn setup(&mut self, _runtime : &mut Runtime) {}

	fn state(&self) -> ServiceState
	{
		match self.state {
			State::None => { return ServiceState::Inactive; }
			State::Error => { return ServiceState::Error; }
			State::KeygenRunning(_, _) => { return ServiceState::Starting; }
			State::Running(_) => { return ServiceState::Running; }
		}
	}

	fn start(&mut self, runtime : &mut Runtime)
	{
		if let State::None = self.state {
			self.state = self.start_keygen(runtime, 0);
		}
	}

	fn stop(&mut self, _runtime : &mut Runtime)
	{
		// TODO: handle killing keygen
		if let State::Running(child) = &mut self.state {
			if let Ok(_) = child.kill() {
				if let Ok(_) = child.wait() {
					self.state = State::None;
					return
				}
			}
			self.state = State::Error;
		} else {
			self.state = State::None;
		}
	}

	fn event(&mut self, runtime : &mut Runtime, event : ServiceEvent) -> bool
	{
		match event {
			ServiceEvent::ProcessExited(pid, _status) => {
				// TODO: error handling
				match &mut self.state {
					State::KeygenRunning(index, child) => {
						if child.id() != pid {
							return false;
						}

						let next = *index + 1;
						self.state = self.start_keygen(runtime, next);
						return true;
					}
					_ => {}
				}
			}
			ServiceEvent::Fd(fd) => {
				if let State::Running(child) = &mut self.state {
					if let Some(stdout) = &mut child.stdout {
						if stdout.as_raw_fd() == fd {
							runtime.logger.service_log("sshd", "got fd event for stdout");
							return true;
						}
					}
					if let Some(stderr) = &mut child.stderr {
						if stderr.as_raw_fd() == fd {
							runtime.logger.service_log("sshd", "got fd event for stderr");
							return true;
						}
					}
				}
			}
			_ => {}
		}
		return false;
	}
}

