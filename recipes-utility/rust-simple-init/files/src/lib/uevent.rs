use std::io;
use std::mem;
use std::collections::HashMap;
use libc;
use std::os::unix::io::RawFd;
use nix::sys::socket::{socket, bind, recvfrom, AddressFamily, SockType, SockAddr};

pub fn netlink_add_group(fd : std::os::unix::io::RawFd, group : i32) -> io::Result<()>
{
	let val : libc::c_int = group;
	unsafe {
		let result = libc::setsockopt(fd,
			libc::SOL_NETLINK, libc::NETLINK_ADD_MEMBERSHIP,
			val as *const libc::c_void, mem::size_of::<libc::c_int>() as u32);
		if result != 0 {
			return Err(std::io::Error::last_os_error());
		}
	}
	return Ok(());
}

fn parse_prop(data : &[u8]) -> Option<(&str, &str)>
{
	if let Ok(s) = std::str::from_utf8(data) {
		let mut parts = s.splitn(2, "=");
		// ignore "action@path"
		if let Some(key) = parts.next() {
			if let Some(value) = parts.next() {
				return Some((key, value));
			}
		}
	}
	return None;
}

fn parse_message(data : &[u8], udev : bool) -> Option<HashMap<String, String>>
{
	let header_size = 8 + (8 * 4);

	if data.len() == 0 {
		return None;
	} else if udev && data.len() < header_size {
		return None;
	}

	let mut offset = 0;

	if udev {
		// let prefix : &[u8] = &data[0..8];
		// std::str::from_utf8(prefix).unwrap()
		offset += header_size;
	}

	let mut properties : HashMap<String, String> = HashMap::new();
	loop {
		if offset >= data.len() {
			break;
		}
		let mut end = offset;
		loop {
			if end >= data.len() || data[end] == 0 {
				break;
			}
			end += 1;
		}
		if let Some((key, val)) = parse_prop(&data[offset..end]) {
			properties.insert(key.to_owned(), val.to_owned());
		}
		offset = end + 1;
	}
	return Some(properties);
}

pub struct Socket
{
	fd : RawFd,
}

impl Socket
{
	pub fn open() -> io::Result<Socket>
	{
		return Socket::open_blocking(true);
	}

	pub fn open_blocking(blocking : bool) -> io::Result<Socket>
	{
		let mut opts = nix::sys::socket::SOCK_CLOEXEC;
		if !blocking {
			opts |= nix::sys::socket::SOCK_NONBLOCK;
		}
		let fd = socket(AddressFamily::Netlink, SockType::Datagram, opts, libc::NETLINK_KOBJECT_UEVENT)?;
		bind(fd, &SockAddr::new_netlink(0, 1 | 2))?; // bind kernel + udev
		return Ok(Socket { fd : fd });
	}

	pub fn read(&self) -> io::Result<Option<HashMap<String, String>>>
	{
		let mut buf = [0u8; 4096];
		let (size, addr) = recvfrom(self.fd, &mut buf)?;
		if size == 0 {
			return Ok(None);
		}

		// check if udev source
		let mut udev : bool = false;
		if let SockAddr::Netlink(nladdr) = addr {
			udev = nladdr.groups() == 2;
		}

		if let Some(props) = parse_message(&buf[0..size], udev) {
			return Ok(Some(props));
		}
		return Ok(None);
	}
}

impl mio::event::Evented for Socket
{
	fn register(&self, poll : &mio::Poll, token: mio::Token, interest: mio::Ready, opts : mio::PollOpt) -> io::Result<()>
	{
		return mio::unix::EventedFd(&self.fd).register(poll, token, interest, opts);
	}

	fn reregister(&self, poll : &mio::Poll, token: mio::Token, interest: mio::Ready, opts : mio::PollOpt) -> io::Result<()>
	{
		return mio::unix::EventedFd(&self.fd).reregister(poll, token, interest, opts);
	}

	fn deregister(&self, poll : &mio::Poll) -> io::Result<()>
	{
		return mio::unix::EventedFd(&self.fd).deregister(poll);
	}
}

pub struct SocketAsync
{
	source : tokio::io::PollEvented<Socket>,
}

impl SocketAsync
{
	pub fn open() -> io::Result<SocketAsync>
	{
		return Ok(SocketAsync { source : tokio::io::PollEvented::new(Socket::open_blocking(false)?)? });
	}

	pub async fn read(&mut self) -> io::Result<Option<HashMap<String, String>>>
	{
		return futures::future::poll_fn(|cx| self.poll_read(cx)).await;
	}

	pub fn poll_read(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<Option<HashMap<String, String>>, io::Error>>
	{
		let ready = mio::Ready::readable();
		futures::ready!(self.source.poll_read_ready(cx, ready))?;
		match self.source.get_ref().read() {
			Ok(event) => std::task::Poll::Ready(Ok(event)),
			Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
				self.source.clear_read_ready(cx, ready)?;
				std::task::Poll::Pending
			}
			Err(e) => std::task::Poll::Ready(Err(e)),
		}
	}
}

pub struct DeviceMonitor<'a>
{
	observers : std::sync::Arc<std::sync::RwLock<Vec<(Option<String>, Box<dyn Fn(&HashMap<String, String>) + Send + Sync + 'a>)>>>,
}

impl<'a> DeviceMonitor<'a>
{
	pub fn new() -> DeviceMonitor<'a>
	{
		return DeviceMonitor { observers : std::sync::Arc::new(std::sync::RwLock::new(Vec::new())) };
	}

	pub fn recv(&self, uevent : &Socket) -> io::Result<()>
	{
		if let Some(event) = uevent.read()? {
			self.process_event(&event);
		}
		return Ok(());
	}

	pub fn process_event(&self, event : &HashMap<String, String>)
	{
		let o = self.observers.read().unwrap();
		for (subsystem, callback) in o.iter() {
			if let Some(name) = subsystem {
				if let Some(eventname) = event.get("SUBSYSTEM") {
					if eventname == name {
						callback(&event);
					}
				}
			} else {
				callback(&event);
			}
		}
	}

	pub fn register_subsystem(&mut self, subsystem : &str, callback : impl Fn(&HashMap<String, String>) + Send + Sync + 'a)
	{
		let mut o = self.observers.write().unwrap();
		o.push((Some(subsystem.to_owned()), Box::new(callback)));
	}
}

