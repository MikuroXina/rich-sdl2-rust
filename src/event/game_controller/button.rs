//! Buttons for a game controller.

use std::{
    ffi::{CStr, CString},
    str::FromStr,
};

use crate::{bind, SdlError};

/// An one of four buttons which be placed like the diamond.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum FourButton {
    /// A button on the up.
    Up,
    /// A button on the right.
    Right,
    /// A button on the down.
    Down,
    /// A button on the left.
    Left,
}

/// A button on a game controller except the trigger buttons.
/// Trigger buttons are covered by [`super::axis::Axis`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum Button {
    /// Four buttons on the left.
    LeftFour(FourButton),
    /// Four buttons on the right.
    RightFour(FourButton),
    /// A back button on the center.
    Back,
    /// A meta button on the center.
    Guide,
    /// A start button on the center.
    Start,
    /// A stick push button on the left.
    LeftStick,
    /// A stick push button on the right.
    RightStick,
    /// A sub trigger button on the left.
    LeftShoulder,
    /// A sub trigger button on the right.
    RightShoulder,
}

impl Button {
    pub(super) fn from_raw(raw: bind::SDL_GameControllerButton) -> Option<Self> {
        use Button::*;
        let val = match raw {
            bind::SDL_CONTROLLER_BUTTON_A => RightFour(FourButton::Down),
            bind::SDL_CONTROLLER_BUTTON_B => RightFour(FourButton::Right),
            bind::SDL_CONTROLLER_BUTTON_X => RightFour(FourButton::Left),
            bind::SDL_CONTROLLER_BUTTON_Y => RightFour(FourButton::Up),
            bind::SDL_CONTROLLER_BUTTON_BACK => Back,
            bind::SDL_CONTROLLER_BUTTON_GUIDE => Guide,
            bind::SDL_CONTROLLER_BUTTON_START => Start,
            bind::SDL_CONTROLLER_BUTTON_LEFTSTICK => LeftStick,
            bind::SDL_CONTROLLER_BUTTON_RIGHTSTICK => RightStick,
            bind::SDL_CONTROLLER_BUTTON_LEFTSHOULDER => LeftShoulder,
            bind::SDL_CONTROLLER_BUTTON_DPAD_DOWN => LeftFour(FourButton::Down),
            bind::SDL_CONTROLLER_BUTTON_DPAD_RIGHT => LeftFour(FourButton::Right),
            bind::SDL_CONTROLLER_BUTTON_DPAD_LEFT => LeftFour(FourButton::Left),
            bind::SDL_CONTROLLER_BUTTON_DPAD_UP => LeftFour(FourButton::Up),
            _ => return None,
        };
        Some(val)
    }

    pub(super) fn as_raw(&self) -> bind::SDL_GameControllerButton {
        use FourButton::*;
        match self {
            Button::LeftFour(Up) => bind::SDL_CONTROLLER_BUTTON_Y,
            Button::LeftFour(Right) => bind::SDL_CONTROLLER_BUTTON_B,
            Button::LeftFour(Down) => bind::SDL_CONTROLLER_BUTTON_A,
            Button::LeftFour(Left) => bind::SDL_CONTROLLER_BUTTON_X,
            Button::RightFour(Up) => bind::SDL_CONTROLLER_BUTTON_DPAD_UP,
            Button::RightFour(Right) => bind::SDL_CONTROLLER_BUTTON_DPAD_RIGHT,
            Button::RightFour(Down) => bind::SDL_CONTROLLER_BUTTON_DPAD_DOWN,
            Button::RightFour(Left) => bind::SDL_CONTROLLER_BUTTON_DPAD_LEFT,
            Button::Back => bind::SDL_CONTROLLER_BUTTON_BACK,
            Button::Guide => bind::SDL_CONTROLLER_BUTTON_GUIDE,
            Button::Start => bind::SDL_CONTROLLER_BUTTON_START,
            Button::LeftStick => bind::SDL_CONTROLLER_BUTTON_LEFTSTICK,
            Button::RightStick => bind::SDL_CONTROLLER_BUTTON_RIGHTSTICK,
            Button::LeftShoulder => bind::SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
            Button::RightShoulder => bind::SDL_CONTROLLER_BUTTON_RIGHTSHOULDER,
        }
    }

    /// Returns the mapping string if exists.
    pub fn to_mapping_string(self) -> Option<String> {
        let ptr = unsafe { bind::SDL_GameControllerGetStringForButton(self.as_raw()) };
        if ptr.is_null() {
            return None;
        }
        let cstr = unsafe { CStr::from_ptr(ptr) };
        Some(cstr.to_string_lossy().to_string())
    }
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c_str =
            unsafe { CStr::from_ptr(bind::SDL_GameControllerGetStringForButton(self.as_raw())) };
        write!(f, "{}", c_str.to_str().unwrap())
    }
}

impl FromStr for Button {
    type Err = SdlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cstr = CString::new(s).map_err(|_| SdlError::Others {
            msg: "string must not be empty".into(),
        })?;
        let axis = unsafe { bind::SDL_GameControllerGetButtonFromString(cstr.as_ptr()) };
        Button::from_raw(axis).ok_or_else(|| SdlError::Others {
            msg: "the axis was not found".into(),
        })
    }
}
