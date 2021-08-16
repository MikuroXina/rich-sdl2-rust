//! Blending for a [`Surface`].

use crate::color::BlendMode;
use crate::{bind, Sdl};

use super::{RawSurface, Surface};

/// A blended [`Surface`].
#[derive(Debug)]
pub struct Blended<S> {
    surface: S,
    mode: BlendMode,
}

impl<S> Blended<S> {
    /// Returns the blend mode.
    pub fn blend_mode(&self) -> &BlendMode {
        &self.mode
    }
}

impl<S: Surface> Blended<S> {
    pub(super) fn new(surface: S, mode: BlendMode) -> Self {
        let raw_mode = mode.into();
        unsafe {
            let ret = bind::SDL_SetSurfaceBlendMode(surface.as_ptr().as_ptr(), raw_mode);
            if ret != 0 {
                Sdl::error_then_panic("Setting surface blend mode");
            }
        }
        Self { surface, mode }
    }
}

impl<S: Surface> Surface for Blended<S> {
    fn as_ptr(&self) -> std::ptr::NonNull<RawSurface> {
        self.surface.as_ptr()
    }
}
