use crate::bind;

pub enum FourButton {
    Up,
    Right,
    Down,
    Left,
}

pub enum Button {
    LeftFour(FourButton),
    RightFour(FourButton),
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
}

impl Button {
    pub(super) fn from_raw(raw: bind::SDL_GameControllerButton) -> Option<Self> {
        use Button::*;
        let val = match raw {
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_A => RightFour(FourButton::Down),
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_B => RightFour(FourButton::Right),
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_X => RightFour(FourButton::Left),
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_Y => RightFour(FourButton::Up),
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_BACK => Back,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_GUIDE => Guide,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_START => Start,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_LEFTSTICK => LeftStick,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_RIGHTSTICK => RightStick,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_LEFTSHOULDER => LeftShoulder,
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_DOWN => {
                LeftFour(FourButton::Down)
            }
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_RIGHT => {
                LeftFour(FourButton::Right)
            }
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_LEFT => {
                LeftFour(FourButton::Left)
            }
            bind::SDL_GameControllerButton_SDL_CONTROLLER_BUTTON_DPAD_UP => {
                LeftFour(FourButton::Up)
            }
            _ => return None,
        };
        Some(val)
    }
}
