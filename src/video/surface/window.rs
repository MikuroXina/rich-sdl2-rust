use std::{marker::PhantomData, ptr::NonNull};

use crate::{bind, window::Window, Sdl};

use super::Surface;

pub struct WindowSurface<'window> {
    surface: NonNull<bind::SDL_Surface>,
    _phantom: PhantomData<&'window Window<'window>>,
}

impl<'window> WindowSurface<'window> {
    pub(crate) fn new(window: &'window Window<'window>) -> Self {
        let surface = unsafe { bind::SDL_GetWindowSurface(window.as_ptr()) };
        Self {
            surface: NonNull::new(surface).unwrap(),
            _phantom: PhantomData,
        }
    }
}

impl Surface for WindowSurface<'_> {
    fn as_ptr(&self) -> std::ptr::NonNull<bind::SDL_Surface> {
        self.surface
    }
}
