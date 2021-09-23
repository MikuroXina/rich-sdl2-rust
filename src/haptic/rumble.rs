//! Rumbling the haptic device.

use crate::{
    bind::{self, SDL_TRUE},
    Result, Sdl, SdlError,
};

use super::Haptic;

/// A controller to rumble the haptic device.
#[derive(Debug)]
pub struct HapticRumble<'haptic> {
    haptic: &'haptic Haptic,
}

impl<'haptic> HapticRumble<'haptic> {
    /// Constructs from a reference to [`Haptic`], or `Err` on failure.
    pub fn new(haptic: &'haptic Haptic) -> Result<Self> {
        let is_supported = unsafe {
            bind::SDL_HapticRumbleSupported(haptic.ptr.as_ptr()) as bind::SDL_bool == SDL_TRUE
        };
        if !is_supported {
            return Err(SdlError::UnsupportedFeature);
        }
        let ret = unsafe { bind::SDL_HapticRumbleInit(haptic.ptr.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self { haptic })
        }
    }

    /// Starts to play a rumbling, or `Err` on failure.
    pub fn play(&self, strength: f32, length: u32) -> Result<()> {
        let ret = unsafe { bind::SDL_HapticRumblePlay(self.haptic.ptr.as_ptr(), strength, length) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Stops playing the rumbling, or `Err` on failure.
    pub fn stop(&self) -> Result<()> {
        let ret = unsafe { bind::SDL_HapticRumbleStop(self.haptic.ptr.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }
}
