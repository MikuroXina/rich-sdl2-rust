use std::ptr::NonNull;

use cloned::ClonedSurface;

use crate::bind;

pub mod cloned;

pub trait Surface {
    fn as_ptr(&self) -> NonNull<bind::SDL_Surface>;

    fn cloned(&self) -> ClonedSurface {
        ClonedSurface::new(self.as_ptr())
    }
}
