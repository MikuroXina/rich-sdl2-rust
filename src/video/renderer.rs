use std::marker::PhantomData;
use std::ptr::NonNull;

use super::window::Window;

use crate::{bind, Sdl};

pub struct Renderer<'window> {
    renderer: NonNull<bind::SDL_Renderer>,
    _phantom: PhantomData<&'window ()>,
}

impl<'window> Renderer<'window> {
    pub fn new(window: &'window Window) -> Self {
        let raw = unsafe { bind::SDL_CreateRenderer(window.as_ptr(), -1, 0) };
        NonNull::new(raw).map_or_else(
            || {
                eprintln!("Sdl renderer error: {}", Sdl::poll_error());
                panic!("Sdl renderer initialization failed");
            },
            |renderer| Self {
                renderer,
                _phantom: PhantomData,
            },
        )
    }
}

impl<'window> Drop for Renderer<'window> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyRenderer(self.renderer.as_ptr()) }
    }
}
