use std::ptr::NonNull;

use crate::bind;

use super::Surface;

pub struct BufferSurface {
    surface: NonNull<bind::SDL_Surface>,
}

impl BufferSurface {
    pub(super) fn new(surface: NonNull<bind::SDL_Surface>) -> Self {
        Self { surface }
    }
}

impl Drop for BufferSurface {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.as_ptr()) }
    }
}

impl Surface for BufferSurface {
    fn as_ptr(&self) -> *mut bind::SDL_Surface {
        self.surface.as_ptr()
    }
}
