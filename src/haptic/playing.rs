//! Playing the effect on the haptic device.

use crate::{bind, Result, Sdl, SdlError};

use super::{effect::HapticEffect, Haptic};

/// An haptic effect but pending to play.
#[derive(Debug)]
pub struct PendingEffect<'haptic> {
    id: i32,
    haptic: &'haptic Haptic,
}

impl<'haptic> PendingEffect<'haptic> {
    pub(super) fn new(id: i32, haptic: &'haptic Haptic) -> Self {
        Self { id, haptic }
    }

    /// Updates the effect with a new effect.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to update the properties.
    pub fn update(&self, effect: &HapticEffect) -> Result<()> {
        let mut raw = effect.clone().into_raw();
        let ret =
            unsafe { bind::SDL_HapticUpdateEffect(self.haptic.ptr.as_ptr(), self.id, &mut raw) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Starts to run the effect. If `iterations` is `None`, the effect repeats over and over indefinitely.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to run the effect on the device.
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

    /// Drops the effect manually.
    pub fn destroy(self) {
        unsafe { bind::SDL_HapticDestroyEffect(self.haptic.ptr.as_ptr(), self.id) }
    }
}

/// A playing haptic effect.
#[derive(Debug)]
pub struct PlayingEffect<'haptic> {
    id: i32,
    haptic: &'haptic Haptic,
}

impl<'haptic> PlayingEffect<'haptic> {
    /// Stops playing the effect.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to stop the effect on the device.
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

    /// Drops the effect manually.
    pub fn destroy(self) {
        unsafe { bind::SDL_HapticDestroyEffect(self.haptic.ptr.as_ptr(), self.id) }
    }
}
