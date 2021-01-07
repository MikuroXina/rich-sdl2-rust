use std::marker::PhantomData;

use crate::{bind, Sdl};

use self::display::Display;

pub mod display;
pub mod geo;
pub mod renderer;
pub mod screen_saver;
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

    pub fn displays(&self) -> Vec<Display> {
        let ret = unsafe { bind::SDL_GetNumVideoDisplays() };
        if ret <= 0 {
            return vec![];
        }
        (0..ret).map(|idx| Display::new(idx, &self)).collect()
    }
}

impl<'sdl> Drop for Video<'sdl> {
    fn drop(&mut self) {
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_VIDEO) }
    }
}
