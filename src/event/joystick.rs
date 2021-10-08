//! A physical joystick device, controlling based on `InputIndex` and raw inputs.

use static_assertions::assert_not_impl_all;
use std::os::raw::c_int;
use std::ptr::NonNull;
use std::{ffi::CStr, marker::PhantomData};

use crate::bind;

use self::axis::Axis;
use self::button::Button;
use self::hat::Hat;
use self::trackball::Trackball;
use self::{
    axis::Axes, button::Buttons, guid::Guid, hat::Hats, power_level::PowerLevel,
    trackball::Trackballs,
};

use super::game_controller::GameController;

pub mod axis;
pub mod button;
pub mod event;
pub mod guid;
pub mod hat;
pub mod power_level;
pub mod trackball;

/// An index on the input device on the joystick. This is only generated from joystick input events from [`crate::EventBox`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputIndex(c_int);

/// An id of the joystick. It can be used to share/duplicate Joystick object, but not [`Send`] and [`Sync`] due to the safety.
#[derive(Clone, PartialEq, Eq)]
pub struct JoystickId<'joystick> {
    pub(super) id: u32,
    pub(super) _phantom: PhantomData<&'joystick Joystick>,
}

assert_not_impl_all!(JoystickId: Send, Sync);

impl std::fmt::Debug for JoystickId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// A physical joystick device manages its states, input devices and power levels.
#[derive(Clone, PartialEq, Eq)]
pub struct Joystick {
    ptr: NonNull<bind::SDL_Joystick>,
}

impl std::fmt::Debug for Joystick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Joystick")
            .field("id", &self.instance_id())
            .finish_non_exhaustive()
    }
}

assert_not_impl_all!(Joystick: Send, Sync);

impl Joystick {
    pub(super) fn new(device_index: u32) -> Self {
        let ptr = unsafe { bind::SDL_JoystickOpen(device_index as c_int) };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    pub(crate) fn ptr(&self) -> &NonNull<bind::SDL_Joystick> {
        &self.ptr
    }

    /// Constructs from [`JoystickId`] if it is valid.
    pub fn from_id(id: JoystickId) -> Option<Self> {
        let ptr = unsafe { bind::SDL_JoystickFromInstanceID(id.id as bind::SDL_JoystickID) };
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Returns the instance id [`JoystickId`].
    pub fn instance_id(&self) -> JoystickId {
        let raw = unsafe { bind::SDL_JoystickInstanceID(self.ptr.as_ptr()) };
        debug_assert_ne!(raw, -1);
        JoystickId {
            id: raw as u32,
            _phantom: PhantomData,
        }
    }

    /// Returns the power level of the joystick.
    pub fn power_level(&self) -> PowerLevel {
        unsafe { bind::SDL_JoystickCurrentPowerLevel(self.ptr.as_ptr()) }.into()
    }

    /// Returns the GUID of the joystick.
    pub fn guid(&self) -> Guid {
        unsafe { bind::SDL_JoystickGetGUID(self.ptr.as_ptr()) }.into()
    }

    /// Returns the name of the joystick.
    pub fn name(&self) -> std::borrow::Cow<str> {
        let c_str = unsafe { CStr::from_ptr(bind::SDL_JoystickName(self.ptr.as_ptr())) };
        c_str.to_string_lossy()
    }

    /// Turns the joystick into enable.
    pub fn enable(&self) {
        unsafe { bind::SDL_JoystickEventState(bind::SDL_ENABLE as c_int) };
    }

    /// Turns the joystick into disable.
    pub fn disable(&self) {
        unsafe { bind::SDL_JoystickEventState(bind::SDL_IGNORE as c_int) };
    }

    /// Returns whether the joystick is enabled.
    pub fn is_enabled(&self) -> bool {
        unsafe { bind::SDL_JoystickEventState(bind::SDL_QUERY) != 0 }
    }

    /// Returns axes [`Axes`] that the joystick has.
    pub fn axes(&self) -> Axes {
        Axes::new(self)
    }

    /// Returns an axis of `index` if it exists.
    pub fn axis(&self, index: InputIndex) -> Option<Axis> {
        let num = unsafe { bind::SDL_JoystickNumAxes(self.ptr.as_ptr()) };
        (index.0 < num).then(|| Axis::new(index, self))
    }

    /// Returns trackballs [`Trackballs`] that the joystick has.
    pub fn trackballs(&self) -> Trackballs {
        Trackballs::new(self)
    }

    /// Returns a trackball of `index` if it exists.
    pub fn trackball(&self, index: InputIndex) -> Option<Trackball> {
        let num = unsafe { bind::SDL_JoystickNumBalls(self.ptr.as_ptr()) };
        (index.0 < num).then(|| Trackball::new(index, self))
    }

    /// Returns buttons [`Buttons`] that the joystick has.
    pub fn buttons(&self) -> Buttons {
        Buttons::new(self)
    }

    /// Returns a button of `index` if it exists.
    pub fn button(&self, index: InputIndex) -> Option<Button> {
        let num = unsafe { bind::SDL_JoystickNumButtons(self.ptr.as_ptr()) };
        (index.0 < num).then(|| Button::new(index, self))
    }

    /// Returns hats [`Hats`] that the joystick has.
    pub fn hats(&self) -> Hats {
        Hats::new(self)
    }

    /// Returns a hat of `index` if it exists.
    pub fn hat(&self, index: InputIndex) -> Option<Hat> {
        let num = unsafe { bind::SDL_JoystickNumHats(self.ptr.as_ptr()) };
        (index.0 < num).then(|| Hat::new(index, self))
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

/// A set of all recognized `Joystick` at initialized.
#[derive(Debug)]
pub struct JoystickSet(Vec<Joystick>);

impl JoystickSet {
    /// Constructs and initializes the system and recognizes joysticks.
    pub fn new() -> Self {
        let num_joysticks = unsafe {
            bind::SDL_InitSubSystem(bind::SDL_INIT_JOYSTICK);
            bind::SDL_NumJoysticks()
        } as u32;
        let joysticks = (0..num_joysticks).map(Joystick::new).collect();
        Self(joysticks)
    }
}

impl Default for JoystickSet {
    fn default() -> Self {
        Self::new()
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
