//! Hats for a physical joystick.

use bitflags::bitflags;
use std::os::raw::c_int;

use crate::bind;

use super::{InputIndex, Joystick};

bitflags! {
    /// A direction of pov hat, representing with a bit flag.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PovHat : u8 {
        /// It is not tilted.
        const CENTERED = 0;
        /// It is on the up.
        const UP = 1 << 0;
        /// It is on the right.
        const RIGHT = 1 << 1;
        /// It is on the down.
        const DOWN = 1 << 2;
        /// It is on the left.
        const LEFT = 1 << 3;
        /// It is on the up and right.
        const RIGHT_UP = Self::RIGHT.bits() | Self::UP.bits();
        /// It is on the down and right.
        const RIGHT_DOWN = Self::RIGHT.bits() | Self::DOWN.bits();
        /// It is on the up and left.
        const LEFT_UP = Self::LEFT.bits() | Self::UP.bits();
        /// It is on the down and left.
        const LEFT_DOWN = Self::LEFT.bits() | Self::DOWN.bits();
    }
}

/// A hat on a physical joystick.
#[derive(Debug)]
pub struct Hat<'joystick> {
    index: c_int,
    joystick: &'joystick Joystick,
}

impl<'joystick> Hat<'joystick> {
    pub(super) fn new(index: InputIndex, joystick: &'joystick Joystick) -> Self {
        Self {
            index: index.0,
            joystick,
        }
    }

    /// Returns the pov hat state of the joystick.
    #[must_use]
    pub fn state(&self) -> PovHat {
        let raw = unsafe { bind::SDL_JoystickGetHat(self.joystick.ptr.as_ptr(), self.index) };
        PovHat::from_bits(raw).expect("hat state must be valid")
    }
}

/// A set of `Hat` for a physical joystick.
#[derive(Debug)]
pub struct Hats<'joystick>(pub Vec<Hat<'joystick>>);

impl<'joystick> Hats<'joystick> {
    pub(super) fn new(joystick: &'joystick Joystick) -> Self {
        let num_hats = unsafe { bind::SDL_JoystickNumHats(joystick.ptr.as_ptr()) };
        let hats = (0..num_hats).map(|index| Hat { index, joystick }).collect();
        Self(hats)
    }
}
