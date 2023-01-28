//! A representation of the power level of the battery in the joystick.

use crate::bind;

/// A power level of a joystick.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
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
        match raw {
            bind::SDL_JOYSTICK_POWER_EMPTY => PowerLevel::Empty,
            bind::SDL_JOYSTICK_POWER_LOW => PowerLevel::Low,
            bind::SDL_JOYSTICK_POWER_MEDIUM => PowerLevel::Medium,
            bind::SDL_JOYSTICK_POWER_FULL => PowerLevel::Full,
            bind::SDL_JOYSTICK_POWER_WIRED => PowerLevel::Wired,
            _ => PowerLevel::Unknown,
        }
    }
}
