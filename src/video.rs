use std::marker::PhantomData;

use crate::{bind, Sdl};

pub mod window;

pub struct Video<'sdl> {
    _phantom: PhantomData<&'sdl ()>,
}

impl<'sdl> Video<'sdl> {
    pub fn new(_: &'sdl Sdl) -> Self {
        let ret = unsafe { bind::SDL_InitSubSystem(bind::SDL_INIT_VIDEO) };
        if ret != 0 {
            eprintln!("Sdl video error: {}", Sdl::poll_error());
            panic!("Sdl video initialization failed");
        }
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'sdl> Drop for Video<'sdl> {
    fn drop(&mut self) {
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_VIDEO) }
    }
}
