//! The SDL_net 2.0 wrapper.

use crate::{bind, Sdl};
use std::{
    ffi::CStr,
    marker::PhantomData,
    mem::MaybeUninit,
    net::{Ipv4Addr, SocketAddrV4},
};

pub mod client;
pub mod conn;
pub mod server;
pub mod sock;

/// A root controller for SDL2_net.
pub struct Net<'sdl> {
    _phantom: PhantomData<&'sdl Sdl>,
}

impl<'sdl> Net<'sdl> {
    /// Constructs a root controller with SDL2 controller.
    pub fn new(_sdl: &'sdl Sdl) -> Self {
        let ret = unsafe { bind::SDLNet_Init() };
        if ret != 0 {
            Sdl::error_then_panic("sdl_net init");
        }
        Self {
            _phantom: PhantomData,
        }
    }

    /// Resolves the ipv4 address to the hostname.
    pub fn resolve_ipv4(&self, addr: Ipv4Addr) -> String {
        let address = bind::IPaddress {
            host: u32::from_ne_bytes(addr.octets()),
            port: 0,
        };
        let cstr = unsafe { CStr::from_ptr(bind::SDLNet_ResolveIP(&address as *const _)) };
        cstr.to_string_lossy().to_string()
    }

    /// Returns all the local addresses of this host's network interfaces.
    pub fn local_addresses(&self) -> Vec<SocketAddrV4> {
        const MAX_ADDRESSES: usize = 16;
        let mut addresses = [MaybeUninit::uninit(); MAX_ADDRESSES];
        let assigned = unsafe {
            bind::SDLNet_GetLocalAddresses(addresses.as_mut_ptr().cast(), MAX_ADDRESSES as _)
        };
        addresses
            .iter()
            .take(assigned as usize)
            .map(|addr| {
                let addr: bind::IPaddress = unsafe { addr.assume_init() };
                SocketAddrV4::new(Ipv4Addr::from(u32::from_be(addr.host)), addr.port)
            })
            .collect()
    }
}

impl Drop for Net<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDLNet_Quit() }
    }
}
