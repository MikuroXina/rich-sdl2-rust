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

impl Rect {
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
        if ret == 0 {
            None
        } else {
            Some(unsafe { raw.assume_init() }.into())
        }
    }

    pub fn has_intersection(&self, other: &Self) -> bool {
        unsafe {
            bind::SDL_HasIntersection(
                &self.clone().into() as *const _,
                &other.clone().into() as *const _,
            ) != 0
        }
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let mut raw = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDL_IntersectRect(
                &self.clone().into() as *const _,
                &other.clone().into() as *const _,
                raw.as_mut_ptr(),
            )
        };
        if ret == 0 {
            None
        } else {
            Some(unsafe { raw.assume_init() }.into())
        }
    }
}
