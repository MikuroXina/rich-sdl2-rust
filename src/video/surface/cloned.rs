use std::ptr::NonNull;

use crate::bind;

use super::Surface;

pub struct ClonedSurface {
    surface: NonNull<bind::SDL_Surface>,
}

impl ClonedSurface {
    pub(super) fn new(src: NonNull<bind::SDL_Surface>) -> Self {
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

impl Surface for ClonedSurface {
    fn as_ptr(&self) -> NonNull<bind::SDL_Surface> {
        self.surface
    }
}

impl Drop for ClonedSurface {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.as_ptr().as_ptr()) }
    }
}
