use crate::{bind, Sdl};

use super::Surface;

pub struct Alpha<S> {
    surface: S,
    alpha: u8,
}

impl<S> Alpha<S> {
    pub fn alpha(&self) -> u8 {
        self.alpha
    }
}

impl<S: Surface> Alpha<S> {
    pub(super) fn new(surface: S, alpha: u8) -> Self {
        unsafe {
            let ret = bind::SDL_SetSurfaceAlphaMod(surface.as_ptr().as_ptr(), alpha);
            if ret != 0 {
                Sdl::error_then_panic("Setting surface alpha mod");
            }
        }
        Self { surface, alpha }
    }
}

impl<S: Surface> Surface for Alpha<S> {
    fn as_ptr(&self) -> std::ptr::NonNull<bind::SDL_Surface> {
        self.surface.as_ptr()
    }
}
