use std::{marker::PhantomData, os::raw::c_int, ptr::NonNull};

use crate::{bind, geo::Rect, window::Window, Result, Sdl, SdlError};

use super::Surface;

pub struct WindowSurface<'window> {
    surface: NonNull<bind::SDL_Surface>,
    window: &'window Window<'window>,
}

impl<'window> WindowSurface<'window> {
    pub(crate) fn new(window: &'window Window<'window>) -> Self {
        let surface = unsafe { bind::SDL_GetWindowSurface(window.as_ptr()) };
        Self {
            surface: NonNull::new(surface).unwrap(),
            window,
        }
    }

    pub fn update_window_surface(&self) -> Result<()> {
        let ret = unsafe { bind::SDL_UpdateWindowSurface(self.window.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

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
    fn as_ptr(&self) -> std::ptr::NonNull<bind::SDL_Surface> {
        self.surface
    }
}
