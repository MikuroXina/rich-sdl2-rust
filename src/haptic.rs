//! Haptic devices, which give the players to feedback by some force.

use bitflags::bitflags;
use std::{ffi::CStr, marker::PhantomData, os::raw::c_int, ptr::NonNull};

use crate::{bind, event::joystick::Joystick, Result, Sdl, SdlError};

pub mod direction;
pub mod effect;
mod joystick;
mod mouse;
mod playing;
pub mod rumble;

pub use joystick::*;
pub use mouse::*;
pub use playing::*;

use self::effect::HapticEffect;

bitflags! {
    /// A property of the haptic device.
    pub struct HapticProperty: u32 {
        /// Supported a constant effect.
        const CONSTANT = 1 << 0;
        /// Supported a sine wave periodic effect.
        const SINE = 1 << 1;
        /// Supported a left/right effect.
        const LEFT_RIGHT = 1 << 2;
        /// Supported a triangle wave periodic effect.
        const TRIANGLE = 1 << 3;
        /// Supported an upwards sawtooth wave periodic effect.
        const SAW_TOOTH_UP = 1 << 4;
        /// Supported a downwards sawtooth wave periodic effect.
        const SAW_TOOTH_DOWN = 1 << 5;
        /// Supported a ramp effect.
        const RAMP = 1 << 6;
        /// Supported a custom effect.
        const CUSTOM = 1 << 11;
        /// Supported setting the global gain.
        const GAIN = 1 << 12;
        /// Supported setting auto-center.
        const AUTO_CENTER = 1 << 13;
        /// Supported querying the status of the effect.
        const STATUS = 1 << 14;
        /// Supported pausing the effect.
        const PAUSE = 1 << 15;
    }
}

/// A haptic device.
pub struct Haptic {
    ptr: NonNull<bind::SDL_Haptic>,
}

impl std::fmt::Debug for Haptic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Haptic")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl Haptic {
    /// Returns the name of the haptic device.
    #[must_use]
    pub fn name(&self) -> String {
        let index = unsafe { bind::SDL_HapticIndex(self.ptr.as_ptr()) };
        let cstr = unsafe { CStr::from_ptr(bind::SDL_HapticName(index)) };
        cstr.to_string_lossy().to_string()
    }

    /// Returns the numbers of the axes on the haptic device.
    #[must_use]
    pub fn num_axes(&self) -> u32 {
        unsafe { bind::SDL_HapticNumAxes(self.ptr.as_ptr()) as u32 }
    }

    /// Returns whether the effect is supported on the haptic device.
    #[must_use]
    pub fn is_effect_supported(&self, effect: &HapticEffect) -> bool {
        let mut raw = effect.clone().into_raw();
        unsafe {
            bind::SDL_HapticEffectSupported(self.ptr.as_ptr(), &mut raw as *mut _) as bind::SDL_bool
                == bind::SDL_TRUE
        }
    }

    /// Constructs the [`PendingEffect`] from the effect specification, or `Err` on failure.
    pub fn new_effect(&self, effect: &HapticEffect) -> Result<PendingEffect> {
        if !self.is_effect_supported(effect) {
            return Err(SdlError::UnsupportedFeature);
        }
        let mut raw = effect.clone().into_raw();
        let ret = unsafe { bind::SDL_HapticNewEffect(self.ptr.as_ptr(), &mut raw as *mut _) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(PendingEffect::new(ret, self))
        }
    }

    /// Returns the capacity of the effects on the haptic device.
    #[must_use]
    pub fn effects_creation_capacity(&self) -> usize {
        unsafe { bind::SDL_HapticNumEffects(self.ptr.as_ptr()) as usize }
    }

    /// Returns the maximum numbers of playing the effects at same time on the haptic device.
    #[must_use]
    pub fn effects_playing_capacity(&self) -> usize {
        unsafe { bind::SDL_HapticNumEffectsPlaying(self.ptr.as_ptr()) as usize }
    }

    /// Stops all the playing effect.
    pub fn stop_all_effect(&self) {
        unsafe {
            bind::SDL_HapticStopAll(self.ptr.as_ptr());
        }
    }

    /// Sets the global gain. If not supported, this has no effects.
    pub fn set_gain(&self, gain: u32) {
        if !self.property().contains(HapticProperty::GAIN) {
            return;
        }
        let ret = unsafe { bind::SDL_HapticSetGain(self.ptr.as_ptr(), gain.min(100) as c_int) };
        if ret < 0 {
            eprintln!("{}", Sdl::error());
        }
    }

    /// Sets auto-center. If not supported, this has no effects.
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

    /// Queries a property on the haptic device.
    #[must_use]
    pub fn property(&self) -> HapticProperty {
        let bits = unsafe { bind::SDL_HapticQuery(self.ptr.as_ptr()) };
        HapticProperty::from_bits(bits).unwrap()
    }

    /// Pauses the haptic device and converts into [`PausedHaptic`].
    #[must_use]
    pub fn pause(self) -> PausedHaptic {
        unsafe {
            bind::SDL_HapticPause(self.ptr.as_ptr());
        }
        PausedHaptic { haptic: self }
    }
}

/// A haptic device but frozen not to interact.
pub struct PausedHaptic {
    haptic: Haptic,
}

impl PausedHaptic {
    /// Unpauses the haptic device and converts into [`Haptic`].
    #[must_use]
    pub fn unpause(self) -> Haptic {
        unsafe {
            bind::SDL_HapticUnpause(self.haptic.ptr.as_ptr());
        }
        self.haptic
    }
}

/// All of recognized haptic devices at initialized.
pub struct HapticSet(Vec<Haptic>);

impl HapticSet {
    /// Constructs and initializes the system and recognizes haptic devices.
    #[must_use]
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

    /// Returns the haptic devices.
    #[must_use]
    pub fn haptics(&self) -> &[Haptic] {
        &self.0
    }
}

impl Default for HapticSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HapticSet {
    fn drop(&mut self) {
        for haptic in &self.0 {
            unsafe { bind::SDL_HapticClose(haptic.ptr.as_ptr()) }
        }
    }
}
