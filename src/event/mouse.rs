use crate::bind;
use crate::geo::Point;

pub mod relative;

#[derive(Debug, Clone)]
pub enum MouseState {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

impl MouseState {
    pub(crate) fn from_bits(bits: u8) -> Self {
        use MouseState::*;
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
    pub state: MouseState,
    pub pos: Point,
    pub move_amount: Point,
}

impl From<bind::SDL_MouseMotionEvent> for MouseMotionEvent {
    fn from(raw: bind::SDL_MouseMotionEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            mouse_id: raw.which,
            state: MouseState::from_bits(raw.state as u8),
            pos: Point { x: raw.x, y: raw.y },
            move_amount: Point {
                x: raw.xrel,
                y: raw.yrel,
            },
        }
    }
}
