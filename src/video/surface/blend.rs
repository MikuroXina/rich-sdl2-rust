use crate::bind;
use crate::color::BlendMode;

use super::Surface;

pub struct Blended<S> {
    surface: S,
    mode: BlendMode,
}

impl<S> Blended<S> {
    pub fn blend_mode(&self) -> &BlendMode {
        &self.mode
    }
}

impl<S: Surface> Blended<S> {
    pub(super) fn new(surface: S, mode: BlendMode) -> Self {
        let raw_mode = mode.clone().into();
        unsafe {
            let _ = bind::SDL_SetSurfaceBlendMode(surface.as_ptr().as_ptr(), raw_mode);
        }
        Self { surface, mode }
    }
}

impl<S: Surface> Surface for Blended<S> {
    fn as_ptr(&self) -> std::ptr::NonNull<bind::SDL_Surface> {
        self.surface.as_ptr()
    }
}
