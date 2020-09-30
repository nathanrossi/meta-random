use std::path::PathBuf;
use std::process::Command;
use super::*;
use super::runtime::{Service, ServiceState, ServiceContext, Runtime};

enum State
{
	None,
	Keygen(usize),
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

	fn check(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		match self.state {
			State::Keygen(keyindex) => {
					if keyindex < self.keys.len() {
						let key = &self.keys[keyindex];

						let keydir = "/etc/ssh";
						if keyindex == 0 {
							if let Err(_) = std::fs::create_dir_all(keydir) {
								println!("[sshd] failed to create key directory {}", keydir);
								self.state = State::Error;
								return;
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
								println!("[sshd] generating {} key", key);
								context.register_child(&child);
								self.state = State::Keygen(keyindex + 1);
							} else {
								println!("[sshd] failed to create key {}", key);
								self.state = State::Error;
								return;
							}
						}
					} else {
						if let Err(_) = std::fs::create_dir_all("/var/run/sshd") {
							println!("[sshd] failed to create run directory");
							self.state = State::Error;
							return;
						}

						if let Ok(child) = Command::new("/usr/sbin/sshd").arg("-D").spawn() {
							println!("[sshd] running (pid = {})", child.id());
							context.register_child(&child);
							self.state = State::Running(child);
						} else {
							println!("[sshd] failed to start sshd");
							self.state = State::Error;
							return;
						}
					}
				}
			_ => {}
		}
	}
}

impl Service for SSHService
{
	fn setup(&mut self, runtime : &Runtime, context : &mut ServiceContext) {}

	fn state(&self) -> ServiceState
	{
		match self.state {
			State::None => { return ServiceState::Inactive; }
			State::Error => { return ServiceState::Error; }
			State::Keygen(_) => { return ServiceState::Starting; }
			State::Running(_) => { return ServiceState::Running; }
			_ => { return ServiceState::Unknown; }
		}
	}

	fn start(&mut self, runtime : &Runtime, context : &mut ServiceContext)
	{
		if let State::None = self.state {
			self.state = State::Keygen(0);
			self.check(runtime, context);
		}
	}

	fn stop(&mut self, runtime : &Runtime, context : &mut ServiceContext)
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

	fn process_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, pid : nix::unistd::Pid, status : std::process::ExitStatus)
	{
		// TODO: error handling
		self.check(runtime, context);
	}

	fn device_event(&mut self, runtime : &Runtime, context : &mut ServiceContext, event : &uevent::EventData) {}
}

