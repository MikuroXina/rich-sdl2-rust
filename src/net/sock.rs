//! Socket to create connection or send a datagram.

use crate::{bind, Result, Sdl, SdlError};
use std::{
    cell::Cell,
    io::{self, Read, Write},
    marker::PhantomData,
    net::{Ipv4Addr, SocketAddrV4},
    os::raw::c_int,
    ptr::NonNull,
};

use super::conn::TcpConnection;

pub mod set;

/// A tcp connection socket for receive packets.
pub struct TcpSocket<'socket> {
    socket: NonNull<bind::_TCPsocket>,
    _phantom: PhantomData<&'socket mut ()>,
}

impl<'socket> TcpSocket<'socket> {
    pub(crate) fn new(address: &'socket mut bind::IPaddress) -> Result<Self> {
        let ptr = unsafe { bind::SDLNet_TCP_Open(address as *mut _) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                socket: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Polls a request from a client, or `None` if no requests received.
    pub fn try_ack(&'socket self) -> Option<TcpConnection<'socket>> {
        let opponent = unsafe { bind::SDLNet_TCP_Accept(self.socket.as_ptr()) };
        NonNull::new(opponent).map(TcpConnection::new)
    }
}

impl Drop for TcpSocket<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_TCP_Close(self.socket.as_ptr()) }
    }
}

const MAX_UDP_CHANNELS: u32 = 32;

/// A udp connection socket for send or receive packets.
pub struct UdpSocket<'socket> {
    socket: NonNull<bind::_UDPsocket>,
    _phantom: PhantomData<&'socket mut ()>,
}

impl<'socket> UdpSocket<'socket> {
    pub(crate) fn new(port: u16) -> Result<Self> {
        let ptr = unsafe { bind::SDLNet_UDP_Open(port) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                socket: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the specific channel of the socket if exists.
    pub fn channel(&self, index: u32) -> Option<UdpChannel> {
        (index <= MAX_UDP_CHANNELS).then(|| UdpChannel::new(index as _, self))
    }

    /// Returns all the channels of the socket.
    pub fn channels(&self) -> Vec<UdpChannel> {
        (0..MAX_UDP_CHANNELS)
            .map(|id| UdpChannel::new(id as _, self))
            .collect()
    }
}

impl Drop for UdpSocket<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_UDP_Close(self.socket.as_ptr()) }
    }
}

/// A channel of a udp socket to matchup packets to specific clients easier.
#[derive(Clone)]
pub struct UdpChannel<'chan> {
    id: c_int,
    socket: &'chan UdpSocket<'chan>,
    bound_addresses: Cell<usize>,
}

impl<'chan> UdpChannel<'chan> {
    fn new(id: c_int, socket: &'chan UdpSocket<'chan>) -> Self {
        Self {
            id,
            socket,
            bound_addresses: Cell::new(0),
        }
    }

    /// Binds the socket address to the channel, or `Err` on failure.
    pub fn bind(&self, address: SocketAddrV4) -> Result<()> {
        let address = bind::IPaddress {
            host: u32::from_ne_bytes(address.ip().octets()),
            port: address.port(),
        };
        let ret = unsafe {
            bind::SDLNet_UDP_Bind(self.socket.socket.as_ptr(), self.id, &address as *const _)
        };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            let count = self.bound_addresses.get() + 1;
            self.bound_addresses.set(count);
            Ok(())
        }
    }

    /// Unbinds the socket address from the channel.
    pub fn unbind(&self) {
        unsafe { bind::SDLNet_UDP_Unbind(self.socket.socket.as_ptr(), self.id) }
    }

    /// Returns the length of the bound addresses.
    pub fn bound_len(&self) -> usize {
        self.bound_addresses.get()
    }

    /// Returns the first bound socket address of the channel.
    pub fn first_bound(&self) -> Option<SocketAddrV4> {
        let ptr = unsafe { bind::SDLNet_UDP_GetPeerAddress(self.socket.socket.as_ptr(), self.id) };
        (!ptr.is_null()).then(|| {
            let address = unsafe { &*ptr };
            SocketAddrV4::new(Ipv4Addr::from(u32::from_be(address.host)), address.port)
        })
    }
}

impl Write for UdpChannel<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut packet = bind::UDPpacket {
            channel: self.id,
            data: buf.as_ptr() as *mut _,
            len: buf.len() as _,
            maxlen: buf.len() as _,
            status: 0,
            address: bind::IPaddress { host: 0, port: 0 },
        };
        let ret = unsafe {
            bind::SDLNet_UDP_Send(self.socket.socket.as_ptr(), self.id, &mut packet as *mut _)
        };
        if ret == 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(ret as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for UdpChannel<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut packet = bind::UDPpacket {
            channel: self.id,
            data: buf.as_mut_ptr(),
            len: buf.len() as _,
            maxlen: buf.len() as _,
            status: 0,
            address: bind::IPaddress { host: 0, port: 0 },
        };
        let ret =
            unsafe { bind::SDLNet_UDP_Recv(self.socket.socket.as_ptr(), &mut packet as *mut _) };
        if ret < 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(ret as usize)
        }
    }
}
