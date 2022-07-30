//! Axes for a physical joystick.

use std::os::raw::c_int;

use crate::bind;

use super::{InputIndex, Joystick};

/// An axis for a physical joystick.
#[derive(Debug)]
pub struct Axis<'joystick> {
    index: c_int,
    joystick: &'joystick Joystick,
}

impl<'joystick> Axis<'joystick> {
    pub(super) fn new(index: InputIndex, joystick: &'joystick Joystick) -> Self {
        Self {
            index: index.0,
            joystick,
        }
    }

    /// Returns the state of axis.
    #[must_use]
    pub fn state(&self) -> i16 {
        unsafe { bind::SDL_JoystickGetAxis(self.joystick.ptr.as_ptr(), self.index) }
    }
}

/// A set of `Axis` for a physical joystick.
#[derive(Debug)]
pub struct Axes<'joystick>(pub Vec<Axis<'joystick>>);

impl<'joystick> Axes<'joystick> {
    pub(super) fn new(joystick: &'joystick Joystick) -> Self {
        let num_axes = unsafe { bind::SDL_JoystickNumAxes(joystick.ptr.as_ptr()) };
        let axes = (0..num_axes)
            .map(|index| Axis { index, joystick })
            .collect();
        Self(axes)
    }
}
