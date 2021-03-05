use crate::{bind, Sdl};

use super::Surface;

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

    pub fn lock(&'surface self) -> RleLock<'surface, S> {
        RleLock::new(&self)
    }
}

pub struct RleLock<'surface, S: Surface> {
    src: &'surface Rle<'surface, S>,
}

impl<'surface, S: Surface> RleLock<'surface, S> {
    fn new(src: &'surface Rle<'surface, S>) -> Self {
        let ret = unsafe { bind::SDL_LockSurface(src.surface.as_ptr().as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("failed to lock RLE surface");
        }
        Self { src }
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
