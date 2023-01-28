//! An event related on a joystick.

use std::{marker::PhantomData, os::raw::c_int};

use crate::{bind, EnumInt};

use super::{hat::PovHat, InputIndex, Joystick, JoystickId};

/// An event occurs on inputted from a joystick or changed a joystick.
#[derive(Debug)]
#[non_exhaustive]
pub enum JoystickEvent<'joystick> {
    /// An Axis was changed,
    Axis {
        /// When this event occurred.
        timestamp: u32,
        /// The id of a joystick that has an axis.
        id: JoystickId<'joystick>,
        /// The index of an axis.
        axis: InputIndex,
        /// The changed value.
        value: i16,
    },
    /// A trackball was changed,
    Trackball {
        /// When this event occurred.
        timestamp: u32,
        /// The id of a joystick that has a trackball.
        id: JoystickId<'joystick>,
        /// The index of a trackball.
        trackball: InputIndex,
        /// The changed value, scroll amounts of x direction.
        x_amount: i16,
        /// The changed value, scroll amounts of y direction.
        y_amount: i16,
    },
    /// A button was changed.
    Button {
        /// When this event occurred.
        timestamp: u32,
        /// The id of a joystick that has a button.
        id: JoystickId<'joystick>,
        /// The index of a button.
        button: InputIndex,
        /// Whether the button was pressed.
        is_pressed: bool,
    },
    /// A hat was changed.
    Hat {
        /// When this event occurred.
        timestamp: u32,
        /// The id of a joystick that has a hat.
        id: JoystickId<'joystick>,
        /// The index of a hat.
        hat: InputIndex,
        /// The changed value.
        value: PovHat,
    },
    /// A joystick was added.
    DeviceAdded {
        /// When this event occurred.
        timestamp: u32,
        /// An added joystick.
        joystick: Joystick,
    },
    /// The joystick was removed.
    DeviceRemoved {
        /// When this event occurred.
        timestamp: u32,
        /// The id of the removed joystick.
        id: JoystickId<'joystick>,
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
            axis: InputIndex(raw.axis as _),
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
            trackball: InputIndex(raw.ball as _),
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
            button: InputIndex(raw.button as _),
            is_pressed: raw.state as u32 == bind::SDL_PRESSED,
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
            hat: InputIndex(raw.hat as _),
            value: PovHat::from_bits(raw.value).unwrap(),
        }
    }
}

impl From<bind::SDL_JoyDeviceEvent> for JoystickEvent<'_> {
    fn from(raw: bind::SDL_JoyDeviceEvent) -> Self {
        let id = JoystickId {
            id: raw.which as u32,
            _phantom: PhantomData,
        };
        match raw.type_ as EnumInt {
            bind::SDL_JOYDEVICEADDED => Self::DeviceAdded {
                timestamp: raw.timestamp,
                joystick: Joystick::from_id(id).unwrap(),
            },
            bind::SDL_JOYDEVICEREMOVED => Self::DeviceRemoved {
                timestamp: raw.timestamp,
                id,
            },
            _ => unreachable!(),
        }
    }
}
