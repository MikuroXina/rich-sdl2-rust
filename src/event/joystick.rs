use static_assertions::assert_not_impl_all;
use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr::NonNull;

use crate::bind;

use self::{
    axis::Axes, button::Buttons, guid::Guid, hat::Hats, power_level::PowerLevel,
    trackball::Trackballs,
};

use super::game_controller::GameController;

pub mod axis;
pub mod button;
pub mod guid;
pub mod hat;
pub mod power_level;
pub mod trackball;

#[derive(Debug, Clone, PartialEq)]
pub struct JoystickId(pub(super) u32);

pub struct Joystick {
    ptr: NonNull<bind::SDL_Joystick>,
}

assert_not_impl_all!(Joystick: Send, Sync);

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

    pub fn guid(&self) -> Guid {
        unsafe { bind::SDL_JoystickGetGUID(self.ptr.as_ptr()) }.into()
    }

    pub fn name(&self) -> std::borrow::Cow<str> {
        let c_str = unsafe { CStr::from_ptr(bind::SDL_JoystickName(self.ptr.as_ptr())) };
        c_str.to_string_lossy()
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

    pub fn axes(&self) -> Axes {
        Axes::new(self)
    }

    pub fn trackballs(&self) -> Trackballs {
        Trackballs::new(self)
    }

    pub fn buttons(&self) -> Buttons {
        Buttons::new(self)
    }

    pub fn hats(&self) -> Hats {
        Hats::new(self)
    }
}

impl From<GameController> for Joystick {
    fn from(gc: GameController) -> Self {
        let ptr = unsafe { bind::SDL_GameControllerGetJoystick(gc.ptr.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
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
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_JOYSTICK) }
    }
}
