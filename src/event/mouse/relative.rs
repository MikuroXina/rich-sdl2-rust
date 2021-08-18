//! Hold the cursor position and change to get relative motions from the mouse.

use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;

use crate::{bind, SdlError, Video};

/// It provides the relative mouse mode, which hiding and holding the cursor and be able to obtain relative motions from the mouse.
/// Dropping this will come back to the normal mouse mode.
pub struct RelativeMouse<'video> {
    video: PhantomData<&'video Video<'video>>,
}

impl std::fmt::Debug for RelativeMouse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RelativeMouse").finish()
    }
}

assert_not_impl_all!(RelativeMouse: Send, Sync);

impl<'video> RelativeMouse<'video> {
    /// Starts the relative mouse mode, but returns `Err(UnsupportedFeature)` if this is not supported.
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
