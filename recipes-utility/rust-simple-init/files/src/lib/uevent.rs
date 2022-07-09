use std::io;
use std::mem;
use std::collections::HashMap;
use std::os::unix::io::{RawFd, AsRawFd};
use libc;
use nix::sys::socket::{socket, bind, recvfrom, AddressFamily, SockType};
use nix::sys::socket::SockaddrLike;
use nix::sys::socket::NetlinkAddr;
use nix::sys::socket::SockFlag;
use nix::sys::socket::SockProtocol;

#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
pub fn netlink_add_group(fd : std::os::unix::io::RawFd, group : i32) -> Result<()>
{
	let val : libc::c_int = group;
	unsafe {
		let result = libc::setsockopt(fd,
			libc::SOL_NETLINK, libc::NETLINK_ADD_MEMBERSHIP,
			val as *const libc::c_void, mem::size_of::<libc::c_int>() as u32);
		if result != 0 {
			return Err(Box::new(std::io::Error::last_os_error()));
		}
	}
	return Ok(());
}

#[derive(Debug)]
pub struct EventData
{
	pub udev : bool,
	pub properties : HashMap<String, String>,
}

impl EventData
{
	pub fn get(&self, key : &str) -> Option<&str>
	{
		if let Some(value) = self.properties.get(key) {
			return Some(&value);
		}
		return None;
	}

	pub fn devpath(&self) -> Option<&str>
	{
		return self.get("DEVPATH");
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

	fn parse_message(data : &[u8], udev : bool) -> Option<EventData>
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
			if let Some((key, val)) = EventData::parse_prop(&data[offset..end]) {
				properties.insert(key.to_owned(), val.to_owned());
			}
			offset = end + 1;
		}
		return Some(EventData { udev : udev, properties : properties });
	}
}

pub struct Socket
{
	fd : RawFd,
}

#[allow(dead_code)]
impl Socket
{
	pub fn open() -> nix::Result<Socket>
	{
		return Socket::open_blocking(true);
	}

	pub fn open_blocking(blocking : bool) -> nix::Result<Socket>
	{
		let mut opts = SockFlag::SOCK_CLOEXEC;
		if !blocking {
			opts |= SockFlag::SOCK_NONBLOCK;
		}
		let fd = socket(AddressFamily::Netlink, SockType::Datagram, opts, SockProtocol::NetlinkKObjectUEvent)?;
		bind(fd, &NetlinkAddr::new(0, 1 | 2))?; // bind kernel + udev
		return Ok(Socket { fd : fd });
	}

	pub fn read(&self) -> nix::Result<Option<EventData>>
	{
		let mut buf = [0u8; 4096];
		match recvfrom::<NetlinkAddr>(self.fd, &mut buf) {
			Ok((size, addr)) => {
					if size == 0 {
						return Ok(None);
					}

					// check if udev source
					let mut udev : bool = false;
					if let Some(nladdr) = addr {
						udev = nladdr.groups() == 2;
					}

					if let Some(data) = EventData::parse_message(&buf[0..size], udev) {
						return Ok(Some(data));
					}
					return Ok(None);
				},
			Err(e) => {
					if let nix::Error::EAGAIN = e {
						return Ok(None);
					}
					return Err(e);
				},
		}
	}
}

impl AsRawFd for Socket
{
	fn as_raw_fd(&self) -> RawFd
	{
		return self.fd;
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

/*
pub struct SocketAsync
{
	source : tokio::io::PollEvented<Socket>,
}

impl SocketAsync
{
	pub fn open() -> Result<SocketAsync>
	{
		return Ok(SocketAsync { source : tokio::io::PollEvented::new(Socket::open_blocking(false)?)? });
	}

	pub async fn read(&mut self) -> Result<Option<EventData>>
	{
		return futures::future::poll_fn(|cx| self.poll_read(cx)).await;
	}

	pub fn poll_read(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<Option<EventData>>>
	{
		let ready = mio::Ready::readable();
		futures::ready!(self.source.poll_read_ready(cx, ready))?;
		match self.source.get_ref().read() {
			Ok(event) => { return std::task::Poll::Ready(Ok(event)); }
			Err(e) => {
				if let nix::Error::Sys(errno) = e {
					if errno == nix::errno::Errno::EAGAIN {
						self.source.clear_read_ready(cx, ready)?;
						return std::task::Poll::Pending;
					}
				}
				return std::task::Poll::Ready(Err(Box::new(e)));
			}
		}
	}
}
*/

