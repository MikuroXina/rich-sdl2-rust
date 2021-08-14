use crate::{bind, Result, Sdl, SdlError};

use super::{effect::HapticEffect, Haptic};

#[derive(Debug)]
pub struct PendingEffect<'haptic> {
    id: i32,
    haptic: &'haptic Haptic,
}

impl<'haptic> PendingEffect<'haptic> {
    pub(super) fn new(id: i32, haptic: &'haptic Haptic) -> Self {
        Self { id, haptic }
    }

    pub fn update(&self, effect: &HapticEffect) -> Result<()> {
        let mut raw = effect.clone().into_raw();
        let ret = unsafe {
            bind::SDL_HapticUpdateEffect(self.haptic.ptr.as_ptr(), self.id, &mut raw as *mut _)
        };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    pub fn run(self, iterations: Option<u32>) -> Result<PlayingEffect<'haptic>> {
        let ret = unsafe {
            bind::SDL_HapticRunEffect(
                self.haptic.ptr.as_ptr(),
                self.id,
                iterations.unwrap_or(bind::SDL_HAPTIC_INFINITY),
            )
        };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(PlayingEffect {
                id: self.id,
                haptic: self.haptic,
            })
        }
    }

    pub fn destroy(self) {
        unsafe { bind::SDL_HapticDestroyEffect(self.haptic.ptr.as_ptr(), self.id) }
    }
}

#[derive(Debug)]
pub struct PlayingEffect<'haptic> {
    id: i32,
    haptic: &'haptic Haptic,
}

impl<'haptic> PlayingEffect<'haptic> {
    pub fn stop(self) -> Result<PendingEffect<'haptic>> {
        let ret = unsafe { bind::SDL_HapticStopEffect(self.haptic.ptr.as_ptr(), self.id) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(PendingEffect {
                id: self.id,
                haptic: self.haptic,
            })
        }
    }

    pub fn destroy(self) {
        unsafe { bind::SDL_HapticDestroyEffect(self.haptic.ptr.as_ptr(), self.id) }
    }
}
