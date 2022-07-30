//! A haptic device integrated with the mouse.

use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use crate::bind;

use super::Haptic;

/// A haptic device got from the mouse.
#[derive(Debug)]
pub struct MouseHaptic {
    haptic: Haptic,
}

impl MouseHaptic {
    /// Constructs if the mouse had the haptic device.
    #[must_use]
    pub fn new() -> Option<Self> {
        let is_supported = unsafe { bind::SDL_MouseIsHaptic() as bind::SDL_bool == bind::SDL_TRUE };
        if !is_supported {
            return None;
        }
        let ptr = unsafe { bind::SDL_HapticOpenFromMouse() };
        Some(Self {
            haptic: Haptic {
                ptr: NonNull::new(ptr).unwrap(),
            },
        })
    }
}

impl Deref for MouseHaptic {
    type Target = Haptic;

    fn deref(&self) -> &Self::Target {
        &self.haptic
    }
}

impl Drop for MouseHaptic {
    fn drop(&mut self) {
        unsafe { bind::SDL_HapticClose(self.ptr.as_ptr()) }
    }
}
