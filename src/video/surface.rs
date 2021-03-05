use std::ptr::NonNull;

use clipped::ClippedSurface;
use cloned::ClonedSurface;

use crate::bind;
use crate::color::BlendMode;
use crate::geo::Rect;

use self::blend::BlendedSurface;

pub mod blend;
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

    fn blend(self, mode: BlendMode) -> BlendedSurface<Self>
    where
        Self: Sized,
    {
        BlendedSurface::new(self, mode)
    }
}
