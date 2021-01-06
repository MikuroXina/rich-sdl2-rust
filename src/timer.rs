use std::marker::PhantomData;

use crate::{bind, Sdl};

pub struct Timer<'sdl> {
    _phantom: PhantomData<&'sdl ()>,
}

impl<'sdl> Timer<'sdl> {
    pub fn new(_: &'sdl Sdl) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn delay(&self, ms: u32) {
        unsafe { bind::SDL_Delay(ms) }
    }
}
