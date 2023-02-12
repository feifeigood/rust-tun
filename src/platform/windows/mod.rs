use std::io::{self, Read, Write};

use std::net::Ipv4Addr;

use crate::error::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct Configuration {}

use crate::configuration::Configuration as C;
use crate::device::Device as D;

/// Create a TUN device with the given name.
pub fn create(_configuration: &C) -> Result<Device> {
    unimplemented!()
}

pub struct Queue {}

#[allow(unused)]
impl Read for Queue {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        todo!()
    }
}

#[allow(unused)]
impl Write for Queue {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        todo!()
    }
}

/// A TUN device using the TUN/TAP Linux driver.
pub struct Device {}

#[allow(unused)]
impl Read for Device {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        todo!()
    }
}

#[allow(unused)]
impl Write for Device {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        todo!()
    }
}

#[allow(unused)]
impl D for Device {
    type Queue = Queue;

    fn name(&self) -> &str {
        todo!()
    }

    fn set_name(&mut self, name: &str) -> Result<()> {
        todo!()
    }

    fn enabled(&mut self, value: bool) -> Result<()> {
        todo!()
    }

    fn address(&self) -> Result<Ipv4Addr> {
        todo!()
    }

    fn set_address(&mut self, value: Ipv4Addr) -> Result<()> {
        todo!()
    }

    fn destination(&self) -> Result<Ipv4Addr> {
        todo!()
    }

    fn set_destination(&mut self, value: Ipv4Addr) -> Result<()> {
        todo!()
    }

    fn broadcast(&self) -> Result<Ipv4Addr> {
        todo!()
    }

    fn set_broadcast(&mut self, value: Ipv4Addr) -> Result<()> {
        todo!()
    }

    fn netmask(&self) -> Result<Ipv4Addr> {
        todo!()
    }

    fn set_netmask(&mut self, value: Ipv4Addr) -> Result<()> {
        todo!()
    }

    fn mtu(&self) -> Result<i32> {
        todo!()
    }

    fn set_mtu(&mut self, value: i32) -> Result<()> {
        todo!()
    }

    fn queue(&mut self, index: usize) -> Option<&mut Self::Queue> {
        todo!()
    }
}
