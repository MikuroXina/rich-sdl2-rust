use std::mem::MaybeUninit;

use crate::bind;

use super::{Point, Size};

/// A rectangle holding up left point and size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[must_use]
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
    /// Returns the center point of the rectangle.
    pub fn center(self) -> Point {
        Point {
            x: self.up_left.x + (self.size.width / 2) as i32,
            y: self.up_left.y + (self.size.height / 2) as i32,
        }
    }

    /// Returns the left x of the rect.
    #[must_use]
    pub fn left(self) -> i32 {
        self.up_left.x
    }
    /// Returns the right x of the rect.
    #[must_use]
    pub fn right(self) -> i32 {
        self.up_left.x + self.size.width as i32
    }
    /// Returns the top y of the rect.
    #[must_use]
    pub fn top(self) -> i32 {
        self.up_left.y
    }
    /// Returns the bottom y of the rect.
    #[must_use]
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
    #[must_use]
    pub fn has_intersection(self, other: Self) -> bool {
        unsafe {
            bind::SDL_HasIntersection(&self.into() as *const _, &other.into() as *const _) != 0
        }
    }

    /// Returns the intersection rectangle of two rectangles.
    #[must_use]
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
    #[must_use]
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

    /// Creates a new rect from the rect with overwriting the new top y position.
    pub fn with_top(self, y: i32) -> Self {
        Self::from_bottom_right(
            self.bottom_right(),
            Size {
                height: y.saturating_sub(self.bottom()).max(0) as u32,
                ..self.size
            },
        )
    }
    /// Creates a new rect from the rect with overwriting the new right x position.
    pub fn with_right(self, x: i32) -> Self {
        Self {
            size: Size {
                width: x.saturating_sub(self.left()).max(0) as u32,
                height: self.size.height,
            },
            ..self
        }
    }
    /// Creates a new rect from the rect with overwriting the new bottom y position.
    pub fn with_bottom(self, y: i32) -> Self {
        Self {
            size: Size {
                width: self.size.width,
                height: self.top().saturating_sub(y) as u32,
            },
            ..self
        }
    }
    /// Creates a new rect from the rect with overwriting the new left x position.
    pub fn with_left(self, x: i32) -> Self {
        Self::from_bottom_right(
            self.bottom_right(),
            Size {
                width: self.right().saturating_sub(x).max(0) as u32,
                ..self.size
            },
        )
    }

    /// Creates a new rect that increased the top from the rect.
    pub fn extend_top(self, increase: u32) -> Self {
        self.with_top(self.top() - increase as i32)
    }
    /// Creates a new rect that increased the right from the rect.
    pub fn extend_right(self, increase: u32) -> Self {
        self.with_right(self.right() + increase as i32)
    }
    /// Creates a new rect that increased the bottom from the rect.
    pub fn extend_bottom(self, increase: u32) -> Self {
        self.with_bottom(self.bottom() + increase as i32)
    }
    /// Creates a new rect that increased the left from the rect.
    pub fn extend_left(self, increase: u32) -> Self {
        self.with_left(self.left() - increase as i32)
    }
}
