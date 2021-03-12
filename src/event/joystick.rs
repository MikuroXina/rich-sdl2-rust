use std::os::raw::c_int;
use std::ptr::NonNull;

use crate::bind;

use self::power_level::PowerLevel;

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

    pub fn power_level(&self) -> PowerLevel {
        unsafe { bind::SDL_JoystickCurrentPowerLevel(self.ptr.as_ptr()) }.into()
    }
}
pub struct JoystickSet(Vec<Joystick>);

impl JoystickSet {
    pub fn init() -> Self {
        let num_joysticks = unsafe {
            bind::SDL_InitSubSystem(bind::SDL_INIT_JOYSTICK);
            bind::SDL_NumJoysticks()
        } as u32;
        let joysticks = (0..num_joysticks)
            .map(|index| {
                let ptr = unsafe { bind::SDL_JoystickOpen(index as c_int) };
                Joystick {
                    recognized_index: index,
                    ptr: NonNull::new(ptr).unwrap(),
                }
            })
            .collect();
        Self(joysticks)
    }
}

impl Drop for JoystickSet {
    fn drop(&mut self) {
        for joystick in &self.0 {
            unsafe { bind::SDL_JoystickClose(joystick.ptr.as_ptr()) }
        }
    }
}
