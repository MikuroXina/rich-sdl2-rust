use crate::bind;
use crate::geo::Point;

pub mod relative;

#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

impl MouseButton {
    pub(crate) fn from_bits(bits: u8) -> Self {
        use MouseButton::*;
        match bits {
            1 => Left,
            2 => Middle,
            3 => Right,
            4 => X1,
            5 => X2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MouseMotionEvent {
    pub timestamp: u32,
    pub window_id: u32,
    pub mouse_id: u32,
    pub button: MouseButton,
    pub pos: Point,
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

#[derive(Debug, Clone)]
pub struct MouseButtonEvent {
    pub timestamp: u32,
    pub window_id: u32,
    pub mouse_id: u32,
    pub button: MouseButton,
    pub is_pressed: bool,
    pub clicks: u8,
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
