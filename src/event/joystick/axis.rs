use std::os::raw::c_int;

use crate::bind;

use super::Joystick;

pub struct Axis<'joystick> {
    index: c_int,
    joystick: &'joystick Joystick,
}

impl<'joystick> Axis<'joystick> {
    pub fn state(&self) -> i16 {
        unsafe { bind::SDL_JoystickGetAxis(self.joystick.ptr.as_ptr(), self.index) }
    }
}

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
