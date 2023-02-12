//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::io;
#[allow(unused)]
use std::io::{IoSlice, Read, Write};

use core::pin::Pin;
use core::task::{Context, Poll};
// use std::ops::Deref;

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
// use tokio_util::codec::Framed;

use crate::platform::Device;
// use crate::TunPacketCodec;

/// An async TUN device wrapper around a TUN device.
pub struct AsyncDevice {
    // inner: Device,
}

#[allow(unused)]
impl AsyncDevice {
    /// Create a new `AsyncDevice` wrapping around a `Device`.
    pub fn new(device: Device) -> io::Result<AsyncDevice> {
        todo!()
        // Ok(AsyncDevice { inner: device })
    }

    /// Returns a shared reference to the underlying Device object
    pub fn get_ref(&self) -> &Device {
        todo!()
        // &self.inner
    }

    /// Returns a mutable reference to the underlying Device object
    pub fn get_mut(&mut self) -> &mut Device {
        todo!()
        // &mut self.inner
    }

    // /// Consumes this AsyncDevice and return a Framed object (unified Stream and Sink interface)
    // pub fn into_framed(self) -> Framed<Self, TunPacketCodec> {
    //     //android false ios macos true, windows 不知道盲猜false
    //     let codec = TunPacketCodec::new(false, 1504);
    //     Framed::new(self, codec)
    // }
}

impl Drop for AsyncDevice {
    fn drop(&mut self) {
        todo!()
        // self.get_mut().session().shutdown();
    }
}

#[allow(unused)]
impl AsyncRead for AsyncDevice {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf,
    ) -> Poll<io::Result<()>> {
        todo!()
        // loop {
        //     if self.inner.is_ready() {
        //         let rbuf = buf.initialize_unfilled();
        //         //let mut guard = ready!(self.inner.read(rbuf))?;
        //         match self.inner.read(rbuf) {
        //             Ok(res) => {
        //                 return Poll::Ready(Ok(buf.advance(res)));
        //             }
        //             Err(e) => {
        //                 if e.kind() == io::ErrorKind::Other {
        //                     println!("[wintun] read package other error");
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}

#[allow(unused)]
impl AsyncWrite for AsyncDevice {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        todo!()
        // loop {
        //     if self.inner.is_ready() {
        //         match self.inner.write(buf) {
        //             Ok(res) => {
        //                 return Poll::Ready(Ok(res));
        //             }
        //             Err(e) => {
        //                 if e.kind() == io::ErrorKind::WouldBlock {
        //                     println!("[wintun] write package error wouldblock?");
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        todo!()
        // Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        todo!()
        // Poll::Ready(Ok(()))
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        todo!()
        // println!("[wintun] write vec slice {}", bufs.len());
        // loop {
        //     let mut n = 0;
        //     for buf in bufs {
        //         let rbuf = buf.deref();
        //         match self.inner.write(rbuf) {
        //             Ok(res) => {
        //                 println!("[wintun] write vec slice buf OK {}", res);
        //                 n += res;
        //             }

        //             Err(e) => {
        //                 if e.kind() == io::ErrorKind::WouldBlock {
        //                     println!("poll_write_vectored package error wouldblock?");
        //                 }
        //                 continue;
        //             }
        //         }
        //     }
        //     return Poll::Ready(Ok(n));
        // }
    }

    fn is_write_vectored(&self) -> bool {
        true
    }
}
