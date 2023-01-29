//! Connections between client and server.

use crate::{bind, Sdl, SdlError};
use std::{
    io::{self, Read, Write},
    marker::PhantomData,
    net::{Ipv4Addr, SocketAddrV4},
    ptr::NonNull,
};

/// A tcp connection.
pub struct TcpConnection<'req> {
    opponent: NonNull<bind::_TCPsocket>,
    _phantom: PhantomData<&'req ()>,
}

impl<'req> TcpConnection<'req> {
    pub(crate) fn new(opponent: NonNull<bind::_TCPsocket>) -> Self {
        Self {
            opponent,
            _phantom: PhantomData,
        }
    }

    /// Returns the socket address of the connected party.
    pub fn address(&self) -> SocketAddrV4 {
        let addr = unsafe { &*bind::SDLNet_TCP_GetPeerAddress(self.opponent.as_ptr()) };
        SocketAddrV4::new(Ipv4Addr::from(u32::from_be(addr.host)), addr.port)
    }
}

impl Drop for TcpConnection<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_TCP_Close(self.opponent.as_ptr()) }
    }
}

impl Read for TcpConnection<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ret = unsafe {
            bind::SDLNet_TCP_Recv(
                self.opponent.as_ptr(),
                buf.as_mut_ptr().cast(),
                buf.len() as _,
            )
        };
        if ret <= 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(ret as _)
        }
    }
}

impl Write for TcpConnection<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let ret = unsafe {
            bind::SDLNet_TCP_Send(self.opponent.as_ptr(), buf.as_ptr().cast(), buf.len() as _)
        };
        if (ret as usize) < buf.len() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(ret as _)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
