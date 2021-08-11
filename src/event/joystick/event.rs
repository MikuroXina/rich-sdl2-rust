use std::marker::PhantomData;

use crate::bind;

use super::{
    axis::Axis,
    button::Button,
    hat::{Hat, PovHat},
    trackball::Trackball,
    Joystick, JoystickId,
};

pub enum JoystickEvent<'joystick> {
    Axis {
        timestamp: u32,
        id: JoystickId<'joystick>,
        value: i16,
    },
    Trackball {
        timestamp: u32,
        id: JoystickId<'joystick>,
        x_amount: i16,
        y_amount: i16,
    },
    Button {
        timestamp: u32,
        id: JoystickId<'joystick>,
        is_pressed: bool,
    },
    DeviceAdded {
        timestamp: u32,
        joystick: Joystick,
    },
    DeviceRemoved {
        timestamp: u32,
        id: JoystickId<'joystick>,
    },
    Hat {
        timestamp: u32,
        id: JoystickId<'joystick>,
        value: PovHat,
    },
}

impl From<bind::SDL_JoyAxisEvent> for JoystickEvent<'_> {
    fn from(raw: bind::SDL_JoyAxisEvent) -> Self {
        Self::Axis {
            timestamp: raw.timestamp,
            id: JoystickId {
                id: raw.which as u32,
                _phantom: PhantomData,
            },
            value: raw.value,
        }
    }
}

impl From<bind::SDL_JoyBallEvent> for JoystickEvent<'_> {
    fn from(raw: bind::SDL_JoyBallEvent) -> Self {
        Self::Trackball {
            timestamp: raw.timestamp,
            id: JoystickId {
                id: raw.which as u32,
                _phantom: PhantomData,
            },
            x_amount: raw.xrel,
            y_amount: raw.yrel,
        }
    }
}

impl From<bind::SDL_JoyButtonEvent> for JoystickEvent<'_> {
    fn from(raw: bind::SDL_JoyButtonEvent) -> Self {
        Self::Button {
            timestamp: raw.timestamp,
            id: JoystickId {
                id: raw.which as u32,
                _phantom: PhantomData,
            },
            is_pressed: raw.state as u32 == bind::SDL_PRESSED,
        }
    }
}

impl From<bind::SDL_JoyDeviceEvent> for JoystickEvent<'_> {
    fn from(raw: bind::SDL_JoyDeviceEvent) -> Self {
        let id = JoystickId {
            id: raw.which as u32,
            _phantom: PhantomData,
        };
        match raw.type_ {
            bind::SDL_EventType_SDL_JOYDEVICEADDED => Self::DeviceAdded {
                timestamp: raw.timestamp,
                joystick: Joystick::from_id(id).unwrap(),
            },
            bind::SDL_EventType_SDL_JOYDEVICEREMOVED => Self::DeviceRemoved {
                timestamp: raw.timestamp,
                id,
            },
            _ => unreachable!(),
        }
    }
}

impl From<bind::SDL_JoyHatEvent> for JoystickEvent<'_> {
    fn from(raw: bind::SDL_JoyHatEvent) -> Self {
        Self::Hat {
            timestamp: raw.timestamp,
            id: JoystickId {
                id: raw.which as u32,
                _phantom: PhantomData,
            },
            value: PovHat::from_bits(raw.value).unwrap(),
        }
    }
}
