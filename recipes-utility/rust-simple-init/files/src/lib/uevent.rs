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
		let fd = socket(AddressFamily::Netlink, SockType::Datagram, nix::sys::socket::SOCK_CLOEXEC, libc::NETLINK_KOBJECT_UEVENT)?;
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

