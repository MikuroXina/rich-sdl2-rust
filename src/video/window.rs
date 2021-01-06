use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::bind;

mod builder;

pub use builder::{WindowBuilder, WindowPos};

pub struct Window<'video> {
    window: NonNull<bind::SDL_Window>,
    _phantom: PhantomData<&'video ()>,
}

impl<'video> Window<'video> {
    pub fn as_ptr(&self) -> *mut bind::SDL_Window {
        self.window.as_ptr()
    }
}

impl<'video> Drop for Window<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyWindow(self.window.as_ptr()) }
    }
}
