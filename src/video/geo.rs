use crate::bind;

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

#[derive(Debug, Clone)]
pub struct Rect {
    pub up_left: Point,
    pub size: Size,
}
