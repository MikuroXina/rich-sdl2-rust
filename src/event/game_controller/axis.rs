use std::{
    ffi::{CStr, CString},
    str::FromStr,
};

use crate::{bind, SdlError};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum Axis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight,
}

impl Axis {
    pub(super) fn from_raw(raw: bind::SDL_GameControllerAxis) -> Option<Self> {
        use Axis::*;
        let val = match raw {
            bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_LEFTX => LeftX,
            bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_LEFTY => LeftY,
            bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_RIGHTX => RightX,
            bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_RIGHTY => RightY,
            bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_TRIGGERLEFT => TriggerLeft,
            bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_TRIGGERRIGHT => TriggerRight,
            _ => return None,
        };
        Some(val)
    }

    pub(super) fn as_raw(&self) -> bind::SDL_GameControllerAxis {
        use Axis::*;
        match self {
            LeftX => bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_LEFTX,
            LeftY => bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_LEFTY,
            RightX => bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_RIGHTX,
            RightY => bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_RIGHTY,
            TriggerLeft => bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_TRIGGERLEFT,
            TriggerRight => bind::SDL_GameControllerAxis_SDL_CONTROLLER_AXIS_TRIGGERRIGHT,
        }
    }

    pub fn to_mapping_string(self) -> Option<String> {
        let ptr = unsafe { bind::SDL_GameControllerGetStringForAxis(self.as_raw()) };
        if ptr.is_null() {
            return None;
        }
        let cstr = unsafe { CStr::from_ptr(ptr) };
        Some(cstr.to_string_lossy().to_string())
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c_str =
            unsafe { CStr::from_ptr(bind::SDL_GameControllerGetStringForAxis(self.as_raw())) };
        write!(f, "{}", c_str.to_str().unwrap())
    }
}

impl FromStr for Axis {
    type Err = SdlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cstr = CString::new(s).map_err(|_| SdlError::Others {
            msg: "string must not be empty".into(),
        })?;
        let axis = unsafe { bind::SDL_GameControllerGetAxisFromString(cstr.as_ptr()) };
        Axis::from_raw(axis).ok_or_else(|| SdlError::Others {
            msg: "the axis was not found".into(),
        })
    }
}
