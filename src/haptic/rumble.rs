use crate::{
    bind::{self, SDL_bool_SDL_TRUE},
    Result, Sdl, SdlError,
};

use super::Haptic;

pub struct HapticRumble<'haptic> {
    haptic: &'haptic Haptic,
}

impl<'haptic> HapticRumble<'haptic> {
    pub fn new(haptic: &'haptic Haptic) -> Result<Self> {
        let is_supported = unsafe {
            bind::SDL_HapticRumbleSupported(haptic.ptr.as_ptr()) as bind::SDL_bool
                == SDL_bool_SDL_TRUE
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

    pub fn play(&self, strength: f32, length: u32) -> Result<()> {
        let ret = unsafe { bind::SDL_HapticRumblePlay(self.haptic.ptr.as_ptr(), strength, length) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    pub fn stop(&self) -> Result<()> {
        let ret = unsafe { bind::SDL_HapticRumbleStop(self.haptic.ptr.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }
}
