use crate::bind;

mod rect;

pub use rect::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<Point> for bind::SDL_Point {
    fn from(Point { x, y }: Point) -> Self {
        use std::os::raw::c_int;
        Self {
            x: x as c_int,
            y: y as c_int,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn clip_with(&mut self, rect: &Rect) {
        unsafe {
            bind::SDL_IntersectRectAndLine(
                &rect.clone().into() as *const _,
                &mut self.start.x as *mut _,
                &mut self.start.y as *mut _,
                &mut self.end.x as *mut _,
                &mut self.end.y as *mut _,
            );
        }
    }
}
