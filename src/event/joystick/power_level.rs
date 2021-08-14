use crate::bind;

#[derive(Debug, Clone, Copy)]
pub enum PowerLevel {
    Unknown,
    Empty,
    Low,
    Medium,
    Full,
    Wired,
    Max,
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
            bind::SDL_JoystickPowerLevel_SDL_JOYSTICK_POWER_MAX => Max,
            _ => Unknown,
        }
    }
}
