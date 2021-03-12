use std::os::raw::c_int;

use crate::bind;

use super::Joystick;

pub struct Button<'joystick> {
    index: c_int,
    joystick: &'joystick Joystick,
}

impl<'joystick> Button<'joystick> {
    pub fn is_pressed(&self) -> bool {
        unsafe { bind::SDL_JoystickGetButton(self.joystick.ptr.as_ptr(), self.index) != 0 }
    }
}

pub struct Buttons<'joystick>(pub Vec<Button<'joystick>>);

impl<'joystick> Buttons<'joystick> {
    pub(super) fn new(joystick: &'joystick Joystick) -> Self {
        let num_buttons = unsafe { bind::SDL_JoystickNumButtons(joystick.ptr.as_ptr()) };
        let buttons = (0..num_buttons)
            .map(|index| Button { index, joystick })
            .collect();
        Self(buttons)
    }
}
