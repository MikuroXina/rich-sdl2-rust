//! Servers for creating the connections.

use crate::{bind, Result, Sdl, SdlError};
use std::{ffi::CString, marker::PhantomData, mem::MaybeUninit, net::Ipv4Addr, ptr::NonNull};

use super::{conn::TcpConnection, sock::UdpSocket, Net};

/// A client to create the connection.
pub struct NetClient<'net> {
    address: bind::IPaddress,
    _phantom: PhantomData<&'net Net<'net>>,
}

impl<'net> NetClient<'net> {
    /// Constructs and ready to start the client socket.
    pub fn new(_net: &'net Net<'net>, address: Ipv4Addr, port: Option<u16>) -> Result<Self> {
        let address_cstr = CString::new(address.to_string()).unwrap();
        let mut address = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDLNet_ResolveHost(
                address.as_mut_ptr(),
                address_cstr.as_ptr(),
                port.unwrap_or(0),
            )
        };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            let address = unsafe { address.assume_init() };
            Ok(Self {
                address,
                _phantom: PhantomData,
            })
        }
    }

    /// Constructs from the hostname and ready to start the client socket.
    pub fn with_hostname(_net: &'net Net<'net>, hostname: &str, port: Option<u16>) -> Result<Self> {
        let hostname_cstr = CString::new(hostname).unwrap();
        let mut address = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDLNet_ResolveHost(
                address.as_mut_ptr(),
                hostname_cstr.as_ptr(),
                port.unwrap_or(0),
            )
        };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            let address = unsafe { address.assume_init() };
            Ok(Self {
                address,
                _phantom: PhantomData,
            })
        }
    }

    /// Opens a tcp connection from the client.
    pub fn open_tcp(&mut self) -> Option<TcpConnection> {
        let opponent = unsafe { bind::SDLNet_TCP_Open(&mut self.address as *mut _) };
        NonNull::new(opponent).map(TcpConnection::new)
    }

    /// Opens a udp socket from the client.
    pub fn open_udp(&mut self) -> Result<UdpSocket> {
        UdpSocket::new(self.address.port)
    }
}
