//! A window treating as a [`Surface`].

use std::{marker::PhantomData, os::raw::c_int, ptr::NonNull};

use crate::{bind, geo::Rect, window::Window, Result, Sdl, SdlError};

use super::{RawSurface, Surface};

/// A [`Surface`] made from the [`Window`].
pub struct WindowSurface<'window> {
    surface: NonNull<RawSurface>,
    window: &'window Window<'window>,
}

impl std::fmt::Debug for WindowSurface<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowSurface")
            .field("window", &self.window)
            .finish_non_exhaustive()
    }
}

impl<'window> WindowSurface<'window> {
    pub(crate) fn new(window: &'window Window<'window>) -> Self {
        let surface = unsafe { bind::SDL_GetWindowSurface(window.as_ptr()) };
        Self {
            surface: NonNull::new(surface).unwrap(),
            window,
        }
    }

    /// Applies the surface into the original window.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to copy the window surface to the screen.
    pub fn update_window_surface(&self) -> Result<()> {
        let ret = unsafe { bind::SDL_UpdateWindowSurface(self.window.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Applies the surface into the original window area only where `rects`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to copy areas of the window surface to the screen.
    pub fn update_window_surface_rects(&self, rects: &[Rect]) -> Result<()> {
        let rects: Vec<_> = rects.iter().map(|&rect| rect.into()).collect();
        let ret = unsafe {
            bind::SDL_UpdateWindowSurfaceRects(
                self.window.as_ptr(),
                rects.as_ptr(),
                rects.len() as c_int,
            )
        };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }
}

impl Surface for WindowSurface<'_> {
    fn as_ptr(&self) -> std::ptr::NonNull<RawSurface> {
        self.surface
    }
}
