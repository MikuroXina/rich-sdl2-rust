use std::ffi::CStr;

use crate::bind;

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
}
