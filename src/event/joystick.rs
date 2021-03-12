use std::os::raw::c_int;
use std::ptr::NonNull;

use crate::bind;

use self::power_level::PowerLevel;

pub mod power_level;

#[derive(Debug, PartialEq)]
pub struct JoystickId(u32);

pub struct Joystick {
    ptr: NonNull<bind::SDL_Joystick>,
}

impl Joystick {
    pub fn from_id(id: JoystickId) -> Option<Self> {
        let ptr = unsafe { bind::SDL_JoystickFromInstanceID(id.0 as bind::SDL_JoystickID) };
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    pub fn instance_id(&self) -> JoystickId {
        let raw = unsafe { bind::SDL_JoystickInstanceID(self.ptr.as_ptr()) };
        debug_assert_ne!(raw, -1);
        JoystickId(raw as u32)
    }

    pub fn power_level(&self) -> PowerLevel {
        unsafe { bind::SDL_JoystickCurrentPowerLevel(self.ptr.as_ptr()) }.into()
    }

    pub fn enable(&self) {
        unsafe { bind::SDL_JoystickEventState(bind::SDL_ENABLE as c_int) };
    }

    pub fn disable(&self) {
        unsafe { bind::SDL_JoystickEventState(bind::SDL_IGNORE as c_int) };
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { bind::SDL_JoystickEventState(bind::SDL_QUERY) != 0 }
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
