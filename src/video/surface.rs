use std::ptr::NonNull;

use clipped::ClippedSurface;
use cloned::ClonedSurface;

use crate::bind;
use crate::geo::Rect;

pub mod clipped;
pub mod cloned;

pub trait Surface {
    fn as_ptr(&self) -> NonNull<bind::SDL_Surface>;

    fn cloned(&self) -> ClonedSurface {
        ClonedSurface::new(self.as_ptr())
    }

    fn clipped(self, area: Rect) -> ClippedSurface<Self>
    where
        Self: Sized,
    {
        ClippedSurface::new(self, area)
    }
}
