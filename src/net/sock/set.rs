//! Socket management by a socket set.

use crate::{bind, net::Net, Sdl};
use std::{os::raw::c_int, ptr::NonNull};

use super::{TcpSocket, UdpSocket};

extern "C" {
    // int _SDLNet_SocketReady(SDLNet_GenericSocket sock);
    fn _SDLNet_SocketReady(sock: bind::SDLNet_GenericSocket) -> c_int;
}

/// A tcp/udp socket to register to [`SocketSet`].
pub enum GeneralSocket<'net> {
    /// A tcp socket.
    Tcp(TcpSocket<'net>),
    /// An udp socket.
    Udp(UdpSocket<'net>),
}

impl GeneralSocket<'_> {
    fn as_general_ptr(&self) -> bind::SDLNet_GenericSocket {
        match self {
            GeneralSocket::Tcp(socket) => socket.socket.as_ptr().cast(),
            GeneralSocket::Udp(socket) => socket.socket.as_ptr().cast(),
        }
    }
}

impl PartialEq for GeneralSocket<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tcp(l0), Self::Tcp(r0)) => l0.socket == r0.socket,
            (Self::Udp(l0), Self::Udp(r0)) => l0.socket == r0.socket,
            _ => false,
        }
    }
}

/// A socket set to observe a socket.
pub struct SocketSet<'net> {
    ptr: NonNull<bind::_SDLNet_SocketSet>,
    sockets: Vec<GeneralSocket<'net>>,
}

impl<'set, 'net: 'set> SocketSet<'set> {
    /// Constructs a new socket set.
    pub fn new(_net: &'net Net<'net>) -> Self {
        Self {
            ptr: NonNull::dangling(),
            sockets: Vec::new(),
        }
    }

    /// Constructs a new socket set with the capacity.
    pub fn with_capacity(_net: &'net Net<'net>, cap: usize) -> Self {
        let ptr = unsafe { bind::SDLNet_AllocSocketSet(cap as _) };
        Self {
            ptr: NonNull::new(ptr).unwrap_or_else(|| Sdl::error_then_panic("alloc socket set")),
            sockets: Vec::with_capacity(cap),
        }
    }

    /// Returns the number of the active sockets in the socket set. Await `timeout` milliseconds until a socket become active.
    pub fn active_sockets_num(&self, timeout: u32) -> usize {
        let ret = unsafe { bind::SDLNet_CheckSockets(self.ptr.as_ptr(), timeout) };
        if ret < 0 {
            Sdl::error_then_panic("get active sockets");
        }
        ret as usize
    }

    /// Returns all the active sockets in the set.
    pub fn active_sockets(&self) -> impl Iterator<Item = &GeneralSocket> {
        self.active_sockets_num(0);
        self.sockets
            .iter()
            .filter(|socket| unsafe { _SDLNet_SocketReady(socket.as_general_ptr()) != 0 })
    }

    /// Reserves capacity for at least `additional` more elements.
    pub fn reserve(&mut self, additional: usize) {
        if self.sockets.capacity() != 0 {
            unsafe { bind::SDLNet_FreeSocketSet(self.ptr.as_ptr()) }
        }
        self.sockets.reserve(additional);
        let ptr = unsafe { bind::SDLNet_AllocSocketSet(self.sockets.capacity() as _) };
        self.ptr = NonNull::new(ptr).unwrap_or_else(|| Sdl::error_then_panic("alloc socket set"));
    }

    /// Appends a tcp/udp socket.
    pub fn push<'socket>(&mut self, socket: GeneralSocket<'socket>)
    where
        'net: 'socket,
        'socket: 'set,
    {
        if self.sockets.len() == self.sockets.capacity() {
            self.reserve(1);
        }
        let _ = unsafe { bind::SDLNet_AddSocket(self.ptr.as_ptr(), socket.as_general_ptr()) };
        self.sockets.push(socket);
    }

    /// Removes a registered socket.
    pub fn remove(&mut self, socket: &GeneralSocket) {
        if let Some(found) = self
            .sockets
            .iter()
            .enumerate()
            .position(|(_, e)| e == socket)
        {
            let _ = unsafe {
                bind::SDLNet_DelSocket(self.ptr.as_ptr(), self.sockets[found].as_general_ptr())
            };
            self.sockets.remove(found);
        }
    }
}

impl Drop for SocketSet<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_FreeSocketSet(self.ptr.as_ptr()) }
    }
}
