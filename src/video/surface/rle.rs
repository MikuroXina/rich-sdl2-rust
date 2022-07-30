//! RLE (Run-Length Encoding) acceleration for a [`Surface`].

use crate::{bind, Sdl};

use super::Surface;

/// A Run-length encoded [`Surface`].
#[derive(Debug)]
pub struct Rle<'surface, S: Surface> {
    surface: &'surface mut S,
}

impl<'surface, S: Surface> Rle<'surface, S> {
    pub(super) fn new(surface: &'surface mut S) -> Self {
        let ret = unsafe { bind::SDL_SetSurfaceRLE(surface.as_ptr().as_ptr(), 1) };
        if ret != 0 {
            Sdl::error_then_panic("invalid surface");
        }
        Self { surface }
    }

    /// Locks the surface and Convert into [`RleLock`].
    pub fn lock(&'surface mut self) -> RleLock<'surface, S> {
        RleLock::new(self)
    }
}

/// A locked RLE surface.
#[derive(Debug)]
pub struct RleLock<'surface, S: Surface> {
    src: &'surface mut Rle<'surface, S>,
}

impl<'surface, S: Surface> RleLock<'surface, S> {
    fn new(src: &'surface mut Rle<'surface, S>) -> Self {
        let ret = unsafe { bind::SDL_LockSurface(src.surface.as_ptr().as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("failed to lock RLE surface");
        }
        Self { src }
    }

    /// Returns the raw pixels data.
    #[must_use]
    pub fn pixels(&self) -> &[u8] {
        let surface = unsafe { self.src.surface.as_ptr().as_ref() };
        let len = surface.h as usize * surface.pitch as usize;
        unsafe { std::slice::from_raw_parts(surface.pixels.cast(), len) }
    }

    /// Returns the raw pixels data for mutating.
    pub fn pixels_mut(&mut self) -> &mut [u8] {
        let surface = unsafe { self.src.surface.as_ptr().as_mut() };
        let len = surface.h as usize * surface.pitch as usize;
        unsafe { std::slice::from_raw_parts_mut(surface.pixels.cast(), len) }
    }
}

impl<'surface, S: Surface> Surface for RleLock<'surface, S> {
    fn as_ptr(&self) -> std::ptr::NonNull<crate::bind::SDL_Surface> {
        self.src.surface.as_ptr()
    }
}

impl<'surface, S: Surface> Drop for RleLock<'surface, S> {
    fn drop(&mut self) {
        unsafe { bind::SDL_UnlockSurface(self.as_ptr().as_ptr()) }
    }
}
