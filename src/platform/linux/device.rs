use std::mem;
use std::ptr;
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use std::ffi::{CString, CStr};
use std::net::{Ipv4Addr};

use libc;
use libc::{c_char};
use libc::{AF_INET, SOCK_DGRAM, O_RDWR};

use error::*;
use device;
use platform::posix::SockAddr;
use platform::linux::sys::*;

/// A TUN device using the TUN/TAP driver.
pub struct Device {
	name: String,
	tun:  RawFd,
	ctl:  RawFd,
}

impl Device {
	pub fn allocate(name: Option<&str>) -> Result<Self> {
		unsafe {
			let dev = match name {
				Some(name) => {
					let name = CString::new(name)?;

					if name.as_bytes_with_nul().len() > IFNAMSIZ {
						return Err(ErrorKind::NameTooLong.into());
					}

					Some(name)
				}

				None =>
					None
			};

			let tun = libc::open(b"/dev/net/tun\0".as_ptr() as *const _, O_RDWR);
			if tun < 0 {
				return Err(io::Error::last_os_error().into());
			}

			let mut req: ifreq = mem::zeroed();
			if let Some(dev) = dev.as_ref() {
				ptr::copy_nonoverlapping(dev.as_ptr() as *const c_char, req.ifrn.name.as_mut_ptr(), dev.as_bytes().len());
			}
			req.ifru.flags = IFF_TUN;

			if tunsetiff(tun, &mut req as *mut _ as *mut _) < 0 {
				libc::close(tun);
				return Err(io::Error::last_os_error().into());
			}

			let ctl = libc::socket(AF_INET, SOCK_DGRAM, 0);
			if ctl < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(Device {
				name: CStr::from_ptr(req.ifrn.name.as_ptr()).to_string_lossy().into(),
				tun:  tun,
				ctl:  ctl,
			})
		}
	}

	unsafe fn request(&self) -> ifreq {
		let mut req: ifreq = mem::zeroed();
		ptr::copy_nonoverlapping(self.name.as_ptr() as *const c_char, req.ifrn.name.as_mut_ptr(), self.name.len());

		req
	}

	/// Make the device persistent.
	pub fn persist(&mut self) -> Result<()> {
		unsafe {
			if tunsetpersist(self.tun, &1) < 0 {
				Err(io::Error::last_os_error().into())
			}
			else {
				Ok(())
			}
		}
	}

	/// Set the owner of the device.
	pub fn user(&mut self, value: i32) -> Result<()> {
		unsafe {
			if tunsetowner(self.tun, &value) < 0 {
				Err(io::Error::last_os_error().into())
			}
			else {
				Ok(())
			}
		}
	}

	/// Set the group of the device.
	pub fn group(&mut self, value: i32) -> Result<()> {
		unsafe {
			if tunsetgroup(self.tun, &value) < 0 {
				Err(io::Error::last_os_error().into())
			}
			else {
				Ok(())
			}
		}
	}
}

impl Read for Device {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		unsafe {
			let amount = libc::read(self.tun, buf.as_mut_ptr() as *mut _, buf.len());

			if amount < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(amount as usize)
		}
	}
}

impl Write for Device {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		unsafe {
			let amount = libc::write(self.tun, buf.as_ptr() as *const _, buf.len());

			if amount < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(amount as usize)
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}

impl device::Device for Device {
	fn name(&self) -> &str {
		&self.name
	}

	fn set_name(&mut self, value: &str) -> Result<()> {
		unsafe {
			let name = CString::new(value)?;

			if name.as_bytes_with_nul().len() > IFNAMSIZ {
				return Err(ErrorKind::NameTooLong.into());
			}

			let mut req = self.request();
			ptr::copy_nonoverlapping(name.as_ptr() as *const c_char, req.ifru.newname.as_mut_ptr(), value.len());

			if siocsifname(self.ctl, &req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			self.name = value.into();

			Ok(())
		}
	}

	fn enabled(&mut self, value: bool) -> Result<()> {
		unsafe {
			let mut req = self.request();

			if siocgifflags(self.ctl, &mut req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			if value {
				req.ifru.flags |= IFF_UP | IFF_RUNNING;
			}
			else {
				req.ifru.flags &= !IFF_UP;
			}

			if siocsifflags(self.ctl, &mut req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(())
		}
	}

	fn address(&self) -> Result<Ipv4Addr> {
		unsafe {
			let mut req = self.request();

			if siocgifaddr(self.ctl, &mut req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			SockAddr::new(&req.ifru.addr).map(Into::into)
		}
	}

	fn set_address(&mut self, value: Ipv4Addr) -> Result<()> {
		unsafe {
			let mut req   = self.request();
			req.ifru.addr = SockAddr::from(value).into();

			if siocsifaddr(self.ctl, &req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(())
		}
	}

	fn destination(&self) -> Result<Ipv4Addr> {
		unsafe {
			let mut req = self.request();

			if siocgifdstaddr(self.ctl, &mut req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			SockAddr::new(&req.ifru.dstaddr).map(Into::into)
		}
	}

	fn set_destination(&mut self, value: Ipv4Addr) -> Result<()> {
		unsafe {
			let mut req      = self.request();
			req.ifru.dstaddr = SockAddr::from(value).into();

			if siocsifdstaddr(self.ctl, &req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(())
		}
	}

	fn broadcast(&self) -> Result<Ipv4Addr> {
		unsafe {
			let mut req = self.request();

			if siocgifbrdaddr(self.ctl, &mut req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			SockAddr::new(&req.ifru.broadaddr).map(Into::into)
		}
	}

	fn set_broadcast(&mut self, value: Ipv4Addr) -> Result<()> {
		unsafe {
			let mut req        = self.request();
			req.ifru.broadaddr = SockAddr::from(value).into();

			if siocsifbrdaddr(self.ctl, &req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(())
		}
	}

	fn netmask(&self) -> Result<Ipv4Addr> {
		unsafe {
			let mut req = self.request();

			if siocgifnetmask(self.ctl, &mut req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			SockAddr::new(&req.ifru.netmask).map(Into::into)
		}
	}

	fn set_netmask(&mut self, value: Ipv4Addr) -> Result<()> {
		unsafe {
			let mut req      = self.request();
			req.ifru.netmask = SockAddr::from(value).into();

			if siocsifnetmask(self.ctl, &req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(())
		}
	}

	fn mtu(&self) -> Result<i32> {
		unsafe {
			let mut req = self.request();

			if siocgifmtu(self.ctl, &mut req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(req.ifru.mtu)
		}
	}

	fn set_mtu(&mut self, value: i32) -> Result<()> {
		unsafe {
			let mut req  = self.request();
			req.ifru.mtu = value;

			if siocsifmtu(self.ctl, &req) < 0 {
				return Err(io::Error::last_os_error().into());
			}

			Ok(())
		}
	}
}

impl AsRawFd for Device {
	fn as_raw_fd(&self) -> RawFd {
		self.tun
	}
}

impl IntoRawFd for Device {
	fn into_raw_fd(self) -> RawFd {
		self.tun
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			libc::close(self.tun);
			libc::close(self.ctl);
		}
	}
}

#[cfg(feature = "mio")]
mod mio {
	use std::io;
	use mio::{Ready, Poll, PollOpt, Token};
	use mio::event::Evented;
	use mio::unix::EventedFd;

	impl Evented for super::Device {
		fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
			EventedFd(&self.tun).register(poll, token, interest, opts)
		}

		fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
			EventedFd(&self.tun).reregister(poll, token, interest, opts)
		}

		fn deregister(&self, poll: &Poll) -> io::Result<()> {
			EventedFd(&self.tun).deregister(poll)
		}
	}
}
