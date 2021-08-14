use std::marker::PhantomData;

use super::{axis::Axis, button::Button};
use crate::{
    bind,
    event::joystick::{Joystick, JoystickId},
};

#[derive(Debug)]
pub enum ControllerEvent<'joystick> {
    Axis {
        timestamp: u32,
        id: JoystickId<'joystick>,
        axis: Axis,
        /// The directions "down" and "right" have positive values here.
        value: i16,
    },
    Button {
        timestamp: u32,
        id: JoystickId<'joystick>,
        button: Button,
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
    DeviceRemapped {
        timestamp: u32,
        id: JoystickId<'joystick>,
    },
}

impl From<bind::SDL_ControllerAxisEvent> for ControllerEvent<'_> {
    fn from(raw: bind::SDL_ControllerAxisEvent) -> Self {
        Self::Axis {
            timestamp: raw.timestamp,
            id: JoystickId {
                id: raw.which as u32,
                _phantom: PhantomData,
            },
            axis: Axis::from_raw(raw.axis as bind::SDL_GameControllerAxis).unwrap(),
            value: raw.value,
        }
    }
}

impl From<bind::SDL_ControllerButtonEvent> for ControllerEvent<'_> {
    fn from(raw: bind::SDL_ControllerButtonEvent) -> Self {
        Self::Button {
            timestamp: raw.timestamp,
            id: JoystickId {
                id: raw.which as u32,
                _phantom: PhantomData,
            },
            button: Button::from_raw(raw.button as bind::SDL_GameControllerButton).unwrap(),
            is_pressed: raw.state as u32 == bind::SDL_PRESSED,
        }
    }
}

impl From<bind::SDL_ControllerDeviceEvent> for ControllerEvent<'_> {
    fn from(raw: bind::SDL_ControllerDeviceEvent) -> Self {
        let id = JoystickId {
            id: raw.which as u32,
            _phantom: PhantomData,
        };
        match raw.type_ {
            bind::SDL_EventType_SDL_CONTROLLERDEVICEADDED => Self::DeviceAdded {
                timestamp: raw.timestamp,
                joystick: Joystick::from_id(id).unwrap(),
            },
            bind::SDL_EventType_SDL_CONTROLLERDEVICEREMOVED => Self::DeviceRemoved {
                timestamp: raw.timestamp,
                id,
            },
            bind::SDL_EventType_SDL_CONTROLLERDEVICEREMAPPED => Self::DeviceRemapped {
                timestamp: raw.timestamp,
                id,
            },
            _ => unreachable!(),
        }
    }
}
