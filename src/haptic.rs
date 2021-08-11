use bitflags::bitflags;
use std::{ffi::CStr, marker::PhantomData, os::raw::c_int, ptr::NonNull};

use crate::{bind, event::joystick::Joystick, Sdl};

mod joystick;
mod mouse;

pub use joystick::*;
pub use mouse::*;

bitflags! {
    pub struct HapticProperty: u32 {
        const CONSTANT = 1 << 0;
        const SINE = 1 << 1;
        const LEFT_RIGHT = 1 << 2;
        const TRIANGLE = 1 << 3;
        const SAW_TOOTH_UP = 1 << 4;
        const SAW_TOOTH_DOWN = 1 << 5;
        const RAMP = 1 << 6;
        const SPRING = 1 << 7;
        const DAMPER = 1 << 8;
        const INERTIA = 1 << 9;
        const FRICTION = 1 << 10;
        const CUSTOM = 1 << 11;
        const GAIN = 1 << 12;
        const AUTO_CENTER = 1 << 13;
        const STATUS = 1 << 14;
        const PAUSE = 1 << 15;
    }
}

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

    pub fn set_gain(&self, gain: u32) {
        if !self.property().contains(HapticProperty::GAIN) {
            return;
        }
        let ret = unsafe { bind::SDL_HapticSetGain(self.ptr.as_ptr(), gain.min(100) as c_int) };
        if ret < 0 {
            eprintln!("{}", Sdl::error());
        }
    }

    pub fn set_auto_center(&self, auto_center: u32) {
        if !self.property().contains(HapticProperty::AUTO_CENTER) {
            return;
        }
        let ret = unsafe {
            bind::SDL_HapticSetAutocenter(self.ptr.as_ptr(), auto_center.min(100) as c_int)
        };
        if ret < 0 {
            eprintln!("{}", Sdl::error());
        }
    }

    pub fn property(&self) -> HapticProperty {
        let bits = unsafe { bind::SDL_HapticQuery(self.ptr.as_ptr()) };
        HapticProperty::from_bits(bits).unwrap()
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
