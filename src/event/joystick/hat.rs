use bitflags::bitflags;
use std::os::raw::c_int;

use crate::bind;

use super::Joystick;

bitflags! {
    pub struct PovHat : u8 {
        const CENTERED = 0;
        const UP = 1 << 0;
        const RIGHT = 1 << 1;
        const DOWN = 1 << 2;
        const LEFT = 1 << 3;
        const RIGHT_UP = Self::RIGHT.bits | Self::UP.bits;
        const RIGHT_DOWN = Self::RIGHT.bits | Self::DOWN.bits;
        const LEFT_UP = Self::LEFT.bits | Self::UP.bits;
        const LEFT_DOWN = Self::LEFT.bits | Self::DOWN.bits;
    }
}

pub struct Hat<'joystick> {
    index: c_int,
    joystick: &'joystick Joystick,
}

impl<'joystick> Hat<'joystick> {
    pub fn state(&self) -> PovHat {
        let raw = unsafe { bind::SDL_JoystickGetHat(self.joystick.ptr.as_ptr(), self.index) };
        PovHat::from_bits(raw).expect("hat state must be valid")
    }
}

pub struct Hats<'joystick>(pub Vec<Hat<'joystick>>);

impl<'joystick> Hats<'joystick> {
    pub(super) fn new(joystick: &'joystick Joystick) -> Self {
        let num_hats = unsafe { bind::SDL_JoystickNumHats(joystick.ptr.as_ptr()) };
        let hats = (0..num_hats).map(|index| Hat { index, joystick }).collect();
        Self(hats)
    }
}
