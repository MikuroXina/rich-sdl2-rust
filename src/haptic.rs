use std::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use crate::{bind, event::joystick::Joystick};

mod joystick;
mod mouse;

pub use joystick::*;
pub use mouse::*;

pub struct Haptic {
    ptr: NonNull<bind::SDL_Haptic>,
}

impl Haptic {
    pub fn name(&self) -> String {
        let index = unsafe { bind::SDL_HapticIndex(self.ptr.as_ptr()) };
        let cstr = unsafe { CStr::from_ptr(bind::SDL_HapticName(index)) };
        cstr.to_string_lossy().to_string()
    }

    pub fn stop_all_effect(&self) {
        unsafe {
            bind::SDL_HapticStopAll(self.ptr.as_ptr());
        }
    }

    pub fn pause(self) -> PausedHaptic {
        unsafe {
            bind::SDL_HapticPause(self.ptr.as_ptr());
        }
        PausedHaptic { haptic: self }
    }
}

pub struct PausedHaptic {
    haptic: Haptic,
}

impl PausedHaptic {
    pub fn unpause(self) -> Haptic {
        unsafe {
            bind::SDL_HapticUnpause(self.haptic.ptr.as_ptr());
        }
        self.haptic
    }
}

#[derive(Default)]
pub struct HapticSet(Vec<Haptic>);

impl HapticSet {
    pub fn new() -> Self {
        let num_haptics = unsafe {
            bind::SDL_InitSubSystem(bind::SDL_INIT_HAPTIC);
            bind::SDL_NumHaptics()
        };
        Self(
            (0..num_haptics)
                .flat_map(|index| {
                    let ptr = unsafe { bind::SDL_HapticOpen(index) };
                    NonNull::new(ptr).map(|ptr| Haptic { ptr })
                })
                .collect(),
        )
    }

    pub fn haptics(&self) -> &[Haptic] {
        &self.0
    }
}

impl Drop for HapticSet {
    fn drop(&mut self) {
        for haptic in &self.0 {
            unsafe { bind::SDL_HapticClose(haptic.ptr.as_ptr()) }
        }
    }
}
