use std::mem::MaybeUninit;

use crate::bind;

use super::{Point, Size};

#[derive(Debug, Clone)]
pub struct Rect {
    pub up_left: Point,
    pub size: Size,
}

impl From<bind::SDL_Rect> for Rect {
    fn from(bind::SDL_Rect { x, y, w, h }: bind::SDL_Rect) -> Self {
        Self {
            up_left: Point { x, y },
            size: Size {
                width: w as u32,
                height: h as u32,
            },
        }
    }
}

impl From<Rect> for bind::SDL_Rect {
    fn from(Rect { up_left, size }: Rect) -> Self {
        use std::os::raw::c_int;
        Self {
            x: up_left.x as c_int,
            y: up_left.y as c_int,
            w: size.width as c_int,
            h: size.height as c_int,
        }
    }
}
