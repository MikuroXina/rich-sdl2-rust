use std::ptr::NonNull;

use crate::bind;

pub mod power_level;

#[derive(Debug, PartialEq)]
pub struct JoystickId(u32);

pub struct Joystick {
    recognized_index: u32,
    ptr: NonNull<bind::SDL_Joystick>,
}

impl Joystick {
    pub fn instance_id(&self) -> JoystickId {
        let raw = unsafe { bind::SDL_JoystickInstanceID(self.ptr.as_ptr()) };
        debug_assert_ne!(raw, -1);
        JoystickId(raw as u32)
    }
}
