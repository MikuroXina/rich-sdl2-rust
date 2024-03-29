//! Cloning a [`Surface`].

use static_assertions::assert_not_impl_all;
use std::ptr::NonNull;

use crate::bind;

use super::{RawSurface, Surface};

/// A cloned [`Surface`].
#[derive(Debug)]
pub struct Cloned {
    surface: NonNull<RawSurface>,
}

assert_not_impl_all!(Cloned: Send, Sync);

impl Cloned {
    pub(super) fn new(src: NonNull<RawSurface>) -> Self {
        let raw = unsafe {
            let src = src.as_ref();
            bind::SDL_CreateRGBSurface(
                0,
                src.w,
                src.h,
                (*src.format).BitsPerPixel.into(),
                (*src.format).Rmask,
                (*src.format).Gmask,
                (*src.format).Bmask,
                (*src.format).Amask,
            )
        };
        Self {
            surface: NonNull::new(raw).expect("invalid source surface"),
        }
    }
}

impl Surface for Cloned {
    fn as_ptr(&self) -> NonNull<RawSurface> {
        self.surface
    }
}

impl Drop for Cloned {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.as_ptr().as_ptr()) }
    }
}
