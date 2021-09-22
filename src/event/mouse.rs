//! Events and utilities for the mouse.

use crate::bind;
use crate::geo::Point;

pub mod cursor;
pub mod relative;

/// A kind of the mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// The left button of the mouse.
    Left,
    /// The middle or wheel button of the mouse.
    Middle,
    /// The right button of the mouse.
    Right,
    /// The x1 of the mouse.
    X1,
    /// The x2 of the mouse.
    X2,
}

impl MouseButton {
    pub(crate) fn from_bits(bits: u8) -> Option<Self> {
        use MouseButton::*;
        Some(match bits {
            1 => Left,
            2 => Middle,
            3 => Right,
            4 => X1,
            5 => X2,
            _ => return None,
        })
    }
}

/// An event related on the mouse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseEvent {
    /// A motion event [`MouseMotionEvent`].
    Motion(MouseMotionEvent),
    /// A button event [`MouseButtonEvent`].
    Button(MouseButtonEvent),
    /// A wheel event [`MouseWheelEvent`].
    Wheel(MouseWheelEvent),
}

/// An event that the mouse was moved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseMotionEvent {
    /// When this event occurred.
    pub timestamp: u32,
    /// The id of the window focused.
    pub window_id: u32,
    /// The id of the moved.
    pub mouse_id: u32,
    /// The button state of the mouse.
    pub button: Option<MouseButton>,
    /// The mouse position.
    pub pos: Point,
    /// The moved amount of the mouse.
    pub move_amount: Point,
}

impl From<bind::SDL_MouseMotionEvent> for MouseMotionEvent {
    fn from(raw: bind::SDL_MouseMotionEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            mouse_id: raw.which,
            button: MouseButton::from_bits(raw.state as u8),
            pos: Point { x: raw.x, y: raw.y },
            move_amount: Point {
                x: raw.xrel,
                y: raw.yrel,
            },
        }
    }
}

/// An event that th mouse button was pressed/released.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseButtonEvent {
    /// When this event occurred.
    pub timestamp: u32,
    /// The id of the window focused.
    pub window_id: u32,
    /// The id of the moved.
    pub mouse_id: u32,
    /// The button state of the mouse.
    pub button: Option<MouseButton>,
    /// Whether the mouse button is pressed.
    pub is_pressed: bool,
    /// The click count of the button.
    pub clicks: u8,
    /// The mouse position.
    pub pos: Point,
}

impl From<bind::SDL_MouseButtonEvent> for MouseButtonEvent {
    fn from(raw: bind::SDL_MouseButtonEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            mouse_id: raw.which,
            button: MouseButton::from_bits(raw.button),
            is_pressed: raw.state as u32 == bind::SDL_PRESSED,
            clicks: raw.clicks,
            pos: Point { x: raw.x, y: raw.y },
        }
    }
}

/// An event that the mouse wheel was scrolled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseWheelEvent {
    /// When this event occurred.
    pub timestamp: u32,
    /// The id of the window focused.
    pub window_id: u32,
    /// The id of the moved.
    pub mouse_id: u32,
    /// How the wheel scrolled.
    pub scroll_amount: Point,
    /// Whether the scroll direction is inverted.
    pub is_flipped: bool,
}

impl From<bind::SDL_MouseWheelEvent> for MouseWheelEvent {
    fn from(raw: bind::SDL_MouseWheelEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            mouse_id: raw.which,
            scroll_amount: Point { x: raw.x, y: raw.y },
            is_flipped: raw.direction == bind::SDL_MOUSEWHEEL_FLIPPED,
        }
    }
}
