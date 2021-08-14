use crate::bind;
use crate::geo::Rect;

use super::Surface;

#[derive(Debug)]
pub struct Clipped<S> {
    surface: S,
    area: Rect,
}

impl<S> Clipped<S> {
    pub fn area(&self) -> Rect {
        self.area
    }
}

impl<S: Surface> Clipped<S> {
    pub(super) fn new(surface: S, area: Rect) -> Self {
        let raw_rect = area.into();
        unsafe {
            let _ = bind::SDL_SetClipRect(surface.as_ptr().as_ptr(), &raw_rect as *const _);
        }
        Self { surface, area }
    }
}

impl<S: Surface> Surface for Clipped<S> {
    fn as_ptr(&self) -> std::ptr::NonNull<bind::SDL_Surface> {
        self.surface.as_ptr()
    }
}
