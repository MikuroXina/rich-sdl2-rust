//! Alpha modification for a [`Surface`].

use crate::{bind, Sdl};

use super::{RawSurface, Surface};

/// An alpha modified [`Surface`].
#[derive(Debug)]
pub struct AlphaMod<S> {
    surface: S,
    alpha: u8,
}

impl<S> AlphaMod<S> {
    /// Returns the alpha modification value.
    pub fn alpha(&self) -> u8 {
        self.alpha
    }
}

impl<S: Surface> AlphaMod<S> {
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

impl<S: Surface> Surface for AlphaMod<S> {
    fn as_ptr(&self) -> std::ptr::NonNull<RawSurface> {
        self.surface.as_ptr()
    }
}
