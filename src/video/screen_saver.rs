use std::marker::PhantomData;

use crate::{bind, Video};

pub struct ScreenSaver<'video> {
    _phantom: PhantomData<&'video ()>,
}

impl<'video> ScreenSaver<'video> {
    pub fn new(_: &'video Video) -> Self {
        unsafe { bind::SDL_EnableScreenSaver() }
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'video> Drop for ScreenSaver<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DisableScreenSaver() }
    }
}
