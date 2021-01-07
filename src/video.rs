use std::marker::PhantomData;

use crate::{bind, Sdl};

pub mod geo;
pub mod renderer;
pub mod window;

pub struct Video<'sdl> {
    _phantom: PhantomData<&'sdl ()>,
}

impl<'sdl> Video<'sdl> {
    pub fn new(_: &'sdl Sdl) -> Self {
        let ret = unsafe { bind::SDL_InitSubSystem(bind::SDL_INIT_VIDEO) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl video")
        }
        Self {
            _phantom: PhantomData,
        }
    }

    // TODO(MikuroXina): screen saver
    // TODO(MikuroXina): display mode
    // TODO(MikuroXina): display stats
    // TODO(MikuroXina): screen saver
}

impl<'sdl> Drop for Video<'sdl> {
    fn drop(&mut self) {
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_VIDEO) }
    }
}
