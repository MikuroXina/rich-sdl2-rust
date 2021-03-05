use std::ptr::NonNull;

use crate::bind;
use crate::color::{BlendMode, Rgb};
use crate::geo::Rect;

pub mod alpha;
pub mod blend;
pub mod clipped;
pub mod cloned;
pub mod color;

use alpha::AlphaMod;
use blend::Blended;
use clipped::Clipped;
use cloned::Cloned;
use color::ColorMod;

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

    fn alpha_mod(self, alpha: u8) -> AlphaMod<Self>
    where
        Self: Sized,
    {
        AlphaMod::new(self, alpha)
    }

    fn color_mod(self, color: Rgb) -> ColorMod<Self>
    where
        Self: Sized,
    {
        ColorMod::new(self, color)
    }
}
