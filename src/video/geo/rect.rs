use std::mem::MaybeUninit;

use crate::bind;

use super::{Point, Size};

/// A rectangle holding up left point and size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rect {
    /// A up left point of the rectangle.
    pub up_left: Point,
    /// A size of the rectangle.
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

impl Rect {
    /// Constructs a rect from the center.
    pub fn from_center(center: Point, size: Size) -> Self {
        Self {
            up_left: Point {
                x: center.x - size.width as i32 / 2,
                y: center.y - size.height as i32 / 2,
            },
            size,
        }
    }
    /// Constructs a rect from the bottom right.
    pub fn from_bottom_right(bottom_right: Point, size: Size) -> Self {
        Self {
            up_left: Point {
                x: (bottom_right.x - size.width as i32),
                y: (bottom_right.y - size.height as i32),
            },
            size,
        }
    }

    /// Constructs a rect from x and y coordinates.
    pub fn from_xs_ys(mut xs: [i32; 2], mut ys: [i32; 2]) -> Self {
        if xs[1] < xs[0] {
            xs.swap(0, 1);
        }
        if ys[1] < ys[0] {
            ys.swap(0, 1);
        }
        Self {
            up_left: Point { x: xs[0], y: ys[0] },
            size: Size {
                width: (xs[1] - xs[0]) as u32,
                height: (ys[1] - ys[0]) as u32,
            },
        }
    }

    /// Returns the bottom right point of the rectangle.
    pub fn bottom_right(self) -> Point {
        Point {
            x: self.up_left.x + self.size.width as i32,
            y: self.up_left.y + self.size.height as i32,
        }
    }
    /// Returns the top right point of the rectangle.
    pub fn top_right(self) -> Point {
        Point {
            x: self.up_left.x + self.size.width as i32,
            y: self.up_left.y,
        }
    }
    /// Returns the bottom left point of the rectangle.
    pub fn bottom_left(self) -> Point {
        Point {
            x: self.up_left.x,
            y: self.up_left.y + self.size.height as i32,
        }
    }

    /// Returns the left x of the rect.
    pub fn left(self) -> i32 {
        self.up_left.x
    }
    /// Returns the right x of the rect.
    pub fn right(self) -> i32 {
        self.up_left.x + self.size.width as i32
    }
    /// Returns the top y of the rect.
    pub fn top(self) -> i32 {
        self.up_left.y
    }
    /// Returns the bottom y of the rect.
    pub fn bottom(self) -> i32 {
        self.up_left.y + self.size.height as i32
    }

    /// Returns the enclosed rectangle of the points, with the clip region.
    pub fn enclosed(points: impl IntoIterator<Item = Point>, clip: Option<Rect>) -> Option<Self> {
        use std::os::raw::c_int;
        let points: Vec<_> = points.into_iter().map(From::from).collect();

        let mut raw = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDL_EnclosePoints(
                points.as_ptr(),
                points.len() as c_int,
                clip.map(From::from)
                    .map_or(std::ptr::null(), |r| &r as *const _),
                raw.as_mut_ptr(),
            )
        };
        (ret != 0).then(|| unsafe { raw.assume_init() }.into())
    }

    /// Returns whether two rectangles intersected.
    pub fn has_intersection(self, other: Self) -> bool {
        unsafe {
            bind::SDL_HasIntersection(&self.into() as *const _, &other.into() as *const _) != 0
        }
    }

    /// Returns the intersection rectangle of two rectangles.
    pub fn intersect(self, other: Self) -> Option<Self> {
        let mut raw = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDL_IntersectRect(
                &self.into() as *const _,
                &other.into() as *const _,
                raw.as_mut_ptr(),
            )
        };
        (ret != 0).then(|| unsafe { raw.assume_init() }.into())
    }

    /// Returns whether the rectangle is empty.
    pub fn is_empty(&self) -> bool {
        self.size.width == 0 || self.size.height == 0
    }

    /// Returns the union of two rectangles.
    pub fn union(self, other: Self) -> Self {
        let mut raw = MaybeUninit::uninit();
        unsafe {
            bind::SDL_UnionRect(
                &self.into() as *const _,
                &other.into() as *const _,
                raw.as_mut_ptr(),
            )
        }
        unsafe { raw.assume_init() }.into()
    }
}
