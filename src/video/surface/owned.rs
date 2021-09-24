//! Owned surface, created from raw pixel data.

use std::{os::raw::c_int, ptr::NonNull};

use super::Surface;
use crate::{
    bind,
    color::pixel::{kind::BppMask, PixelFormat},
    geo::Size,
    Result, Sdl, SdlError,
};

/// An owned [`Surface`] with raw pixel data.
#[derive(Debug)]
pub struct Owned {
    raw: NonNull<bind::SDL_Surface>,
}

impl Owned {
    /// Creates a new owned surface with its size and pixel format.
    pub fn new(size: Size, kind: PixelFormat) -> Result<Self> {
        let BppMask {
            r_mask,
            g_mask,
            b_mask,
            a_mask,
            ..
        } = kind.kind().to_bpp_mask().unwrap_or_default();
        let ptr = unsafe {
            bind::SDL_CreateRGBSurface(
                0,
                size.width as c_int,
                size.height as c_int,
                kind.bits_per_pixel() as c_int,
                r_mask,
                g_mask,
                b_mask,
                a_mask,
            )
        };
        NonNull::new(ptr).map_or_else(
            || Err(SdlError::Others { msg: Sdl::error() }),
            |raw| Ok(Self { raw }),
        )
    }
}

impl Drop for Owned {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.raw.as_ptr()) }
    }
}

impl Surface for Owned {
    fn as_ptr(&self) -> std::ptr::NonNull<super::RawSurface> {
        self.raw
    }
}
