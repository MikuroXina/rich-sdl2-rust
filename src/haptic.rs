use std::{marker::PhantomData, ptr::NonNull};

use crate::{bind, event::joystick::Joystick};

mod joystick;
mod mouse;

pub use joystick::*;
pub use mouse::*;

pub struct Haptic {
    ptr: NonNull<bind::SDL_Haptic>,
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
