use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::bind;
use crate::window::Window;

use super::Surface;

pub struct WindowSurface<'window> {
    surface: NonNull<bind::SDL_Surface>,
    _phantom: PhantomData<&'window mut ()>,
}

impl<'window> WindowSurface<'window> {
    pub fn new(window: &'window mut Window<'window>) -> Self {
        let raw_surface = unsafe { bind::SDL_GetWindowSurface(window.as_ptr()) };
        let surface = NonNull::new(raw_surface).unwrap();
        Self {
            surface,
            _phantom: PhantomData,
        }
    }
}

impl Surface for WindowSurface<'_> {
    fn as_ptr(&self) -> *mut bind::SDL_Surface {
        self.surface.as_ptr()
    }
}
