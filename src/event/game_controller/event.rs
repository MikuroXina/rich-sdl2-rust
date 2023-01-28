//! An event related on a game controller.

use std::marker::PhantomData;

use super::{axis::Axis, button::Button};
use crate::{
    bind,
    event::joystick::{Joystick, JoystickId},
    EnumInt,
};

/// An event occurs on inputted from a game controller or changed a game controller.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ControllerEvent<'joystick> {
    /// An axis was changed.
    Axis {
        /// When this event occurred.
        timestamp: u32,
        /// An id of the joystick having this axis.
        id: JoystickId<'joystick>,
        /// A changed axis.
        axis: Axis,
        /// The changed value. The directions "down" and "right" are positive.
        value: i16,
    },
    /// A button was changed.
    Button {
        /// When this event occurred.
        timestamp: u32,
        /// An id of the joystick having this button.
        id: JoystickId<'joystick>,
        /// A changed button.
        button: Button,
        /// Whether the button was pressed.
        is_pressed: bool,
    },
    /// A game controller was added.
    DeviceAdded {
        /// When this event occurred.
        timestamp: u32,
        /// An added joystick.
        joystick: Joystick,
    },
    /// The game controller was removed.
    DeviceRemoved {
        /// When this event occurred.
        timestamp: u32,
        /// The id of the removed joystick.
        id: JoystickId<'joystick>,
    },
    /// The game controller was remapped.
    DeviceRemapped {
        /// When this event occurred.
        timestamp: u32,
        /// The id of the remapped joystick.
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
        match raw.type_ as EnumInt {
            bind::SDL_CONTROLLERDEVICEADDED => Self::DeviceAdded {
                timestamp: raw.timestamp,
                joystick: Joystick::from_id(id).unwrap(),
            },
            bind::SDL_CONTROLLERDEVICEREMOVED => Self::DeviceRemoved {
                timestamp: raw.timestamp,
                id,
            },
            bind::SDL_CONTROLLERDEVICEREMAPPED => Self::DeviceRemapped {
                timestamp: raw.timestamp,
                id,
            },
            _ => unreachable!(),
        }
    }
}
