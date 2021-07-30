use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;

use crate::{bind, SdlError, Video};

pub struct RelativeMouse<'video> {
    video: PhantomData<&'video Video<'video>>,
}

assert_not_impl_all!(RelativeMouse: Send, Sync);

impl<'video> RelativeMouse<'video> {
    pub fn new(_: &'video Video) -> Result<Self, SdlError> {
        let ret = unsafe { bind::SDL_SetRelativeMouseMode(bind::SDL_bool_SDL_TRUE) };
        if ret == -1 {
            return Err(SdlError::UnsupportedFeature);
        }
        Ok(Self { video: PhantomData })
    }
}

impl Drop for RelativeMouse<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = bind::SDL_SetRelativeMouseMode(bind::SDL_bool_SDL_FALSE);
        }
    }
}
