use std::ptr::NonNull;

use clipped::Clipped;
use cloned::Cloned;

use crate::bind;
use crate::color::BlendMode;
use crate::geo::Rect;

use self::alpha::Alpha;
use self::blend::Blended;

pub mod alpha;
pub mod blend;
pub mod clipped;
pub mod cloned;

pub trait Surface {
    fn as_ptr(&self) -> NonNull<bind::SDL_Surface>;

    fn cloned(&self) -> Cloned {
        Cloned::new(self.as_ptr())
    }

    fn clipped(self, area: Rect) -> Clipped<Self>
    where
        Self: Sized,
    {
        Clipped::new(self, area)
    }

    fn blend(self, mode: BlendMode) -> Blended<Self>
    where
        Self: Sized,
    {
        Blended::new(self, mode)
    }

    fn alpha_mod(self, alpha: u8) -> Alpha<Self>
    where
        Self: Sized,
    {
        Alpha::new(self, alpha)
    }
}
