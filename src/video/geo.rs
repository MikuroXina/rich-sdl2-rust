use crate::bind;

mod rect;

pub use rect::*;

/// A point of the cartesian coordinate system.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    /// An x coordinate of the point.
    pub x: i32,
    /// An y coordinate of the point.
    pub y: i32,
}

impl From<bind::SDL_Point> for Point {
    fn from(bind::SDL_Point { x, y }: bind::SDL_Point) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
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

impl Point {
    /// Returns whether the point is in the rectangle.
    pub fn is_in(&self, rect: Rect) -> bool {
        let bottom_right = rect.bottom_right();
        rect.up_left.x <= self.x
            && self.x <= bottom_right.x
            && rect.up_left.y <= self.y
            && self.y <= bottom_right.y
    }
}

/// A geometry size representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
    /// A width of the geometry.
    pub width: u32,
    /// A height of the geometry.
    pub height: u32,
}

/// A scale from the normal geometry.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Scale {
    /// A horizontal scale, normalized.
    pub horizontal: f32,
    /// A vertical scale, normalized.
    pub vertical: f32,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            horizontal: 1.0,
            vertical: 1.0,
        }
    }
}

/// A straight line from point to point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    /// A start point of the line.
    pub start: Point,
    /// A end point of the line.
    pub end: Point,
}

impl Line {
    /// Clips the line with a rect.
    pub fn clip_with(mut self, rect: Rect) -> Self {
        unsafe {
            bind::SDL_IntersectRectAndLine(
                &(rect.into()) as *const _,
                &mut self.start.x as *mut _,
                &mut self.start.y as *mut _,
                &mut self.end.x as *mut _,
                &mut self.end.y as *mut _,
            );
        }
        self
    }
}
