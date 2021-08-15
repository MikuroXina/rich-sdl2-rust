use crate::bind;

/// A power level of a joystick.
#[derive(Debug, Clone, Copy)]
pub enum PowerLevel {
    /// A power level is unavailable.
    Unknown,
    /// The power level is a few left.
    Empty,
    /// The power level is on low.
    Low,
    /// The power level is on medium.
    Medium,
    /// The power level is full.
    Full,
    /// The power is coming from wired.
    Wired,
}

impl From<bind::SDL_JoystickPowerLevel> for PowerLevel {
    fn from(raw: bind::SDL_JoystickPowerLevel) -> Self {
        use PowerLevel::*;
        match raw {
            bind::SDL_JoystickPowerLevel_SDL_JOYSTICK_POWER_EMPTY => Empty,
            bind::SDL_JoystickPowerLevel_SDL_JOYSTICK_POWER_LOW => Low,
            bind::SDL_JoystickPowerLevel_SDL_JOYSTICK_POWER_MEDIUM => Medium,
            bind::SDL_JoystickPowerLevel_SDL_JOYSTICK_POWER_FULL => Full,
            bind::SDL_JoystickPowerLevel_SDL_JOYSTICK_POWER_WIRED => Wired,
            _ => Unknown,
        }
    }
}
