//! Servers for receiving the connections.

use crate::{bind, Result, Sdl, SdlError};
use std::{marker::PhantomData, mem::MaybeUninit};

use super::{sock::TcpSocket, Net};

/// A server to serve the connection.
pub struct NetServer<'net> {
    address: bind::IPaddress,
    _phantom: PhantomData<&'net Net<'net>>,
}

impl<'net> NetServer<'net> {
    /// Constructs and ready to start the server socket.
    pub fn new(_net: &'net Net<'net>, port: u16) -> Result<Self> {
        let mut address = MaybeUninit::uninit();
        let ret = unsafe { bind::SDLNet_ResolveHost(address.as_mut_ptr(), std::ptr::null(), port) };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            let mut address = unsafe { address.assume_init() };
            address.port = port;
            Ok(Self {
                address,
                _phantom: PhantomData,
            })
        }
    }

    /// Opens a tcp connection socket.
    pub fn open_tcp(&'net mut self) -> Result<TcpSocket<'net>> {
        TcpSocket::new(&mut self.address)
    }
}
