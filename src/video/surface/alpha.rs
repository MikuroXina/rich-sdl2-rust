use crate::bind;

use super::Surface;

pub struct AlphaSurface<S> {
    surface: S,
    alpha: u8,
}

impl<S> AlphaSurface<S> {
    pub fn alpha(&self) -> u8 {
        self.alpha
    }
}

impl<S: Surface> AlphaSurface<S> {
    pub(super) fn new(surface: S, alpha: u8) -> Self {
        unsafe {
            let _ = bind::SDL_SetSurfaceAlphaMod(surface.as_ptr().as_ptr(), alpha);
        }
        Self { surface, alpha }
    }
}

impl<S: Surface> Surface for AlphaSurface<S> {
    fn as_ptr(&self) -> std::ptr::NonNull<bind::SDL_Surface> {
        self.surface.as_ptr()
    }
}
