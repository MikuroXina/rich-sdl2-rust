use std::convert::TryFrom;

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
}
