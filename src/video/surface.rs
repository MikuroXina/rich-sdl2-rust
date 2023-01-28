//! Surface, providing flexible modification for the pixels.

use std::ptr::NonNull;

use crate::color::pixel::Pixel;
use crate::color::{BlendMode, Rgb};
use crate::geo::{Point, Rect};
use crate::{as_raw, color::pixel::palette::Palette};
use crate::{bind, Sdl};
pub use bind::SDL_Surface as RawSurface;

pub mod alpha;
pub mod blend;
pub mod bmp;
pub mod clipped;
pub mod cloned;
pub mod color;
pub mod owned;
pub mod rle;
pub mod window;

use alpha::AlphaMod;
use blend::Blended;
use clipped::Clipped;
use cloned::Cloned;
use color::ColorMod;
use rle::Rle;

/// A trait that provides flexible modification methods.
pub trait Surface {
    /// Returns the raw non-null pointer.
    fn as_ptr(&self) -> NonNull<RawSurface>;

    /// Clones the surface.
    fn cloned(&self) -> Cloned {
        Cloned::new(self.as_ptr())
    }

    /// Clips the surface by `area`.
    fn clipped(self, area: Rect) -> Clipped<Self>
    where
        Self: Sized,
    {
        Clipped::new(self, area)
    }

    /// Changes blend mode of the surface.
    fn blend(self, mode: BlendMode) -> Blended<Self>
    where
        Self: Sized,
    {
        Blended::new(self, mode)
    }

    /// Modifies the alpha of the surface.
    fn alpha_mod(self, alpha: u8) -> AlphaMod<Self>
    where
        Self: Sized,
    {
        AlphaMod::new(self, alpha)
    }

    /// Modifies the color of the surface.
    fn color_mod(self, color: Rgb) -> ColorMod<Self>
    where
        Self: Sized,
    {
        ColorMod::new(self, color)
    }

    /// Fills in the `area` with the `color`, or whole if `area` is `None`.
    fn fill_rect(&self, area: Option<Rect>, color: Pixel) {
        let area = area.map(Into::into);
        unsafe {
            let ret = bind::SDL_FillRect(self.as_ptr().as_ptr(), as_raw(&area), color.as_u32());
            if ret != 0 {
                Sdl::error_then_panic("Surface filling with rectangle")
            }
        }
    }

    /// Fills in the `areas` with the `color`.
    fn fill_rects(&self, areas: impl IntoIterator<Item = Rect>, color: Pixel) {
        let raw_rects: Vec<_> = areas.into_iter().map(Into::into).collect();
        unsafe {
            let ret = bind::SDL_FillRects(
                self.as_ptr().as_ptr(),
                raw_rects.as_ptr(),
                raw_rects.len() as i32,
                color.as_u32(),
            );
            if ret != 0 {
                Sdl::error_then_panic("Surface filling with rectangles")
            }
        }
    }

    /// Overwrites the palette of the surface.
    fn set_palette(&self, palette: &Palette) {
        let ret = unsafe { bind::SDL_SetSurfacePalette(self.as_ptr().as_ptr(), palette.as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Surface setting palette");
        }
    }

    /// Copies `src_area` area in the surface into `dst_pos` on another surface.
    fn copy_to<S: Surface>(&self, src_area: Rect, dst: &S, dst_pos: Point) {
        let src_rect = src_area.into();
        let mut dst_rect = bind::SDL_Rect {
            x: dst_pos.x,
            y: dst_pos.y,
            w: 0,
            h: 0,
        };
        let ret = unsafe {
            bind::SDL_UpperBlit(
                self.as_ptr().as_ptr(),
                &src_rect,
                dst.as_ptr().as_ptr(),
                &mut dst_rect,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Surface copying to another");
        }
    }

    /// Run-length encodes the surface.
    fn rle(&'_ mut self) -> Rle<'_, Self>
    where
        Self: Sized,
    {
        Rle::new(self)
    }
}
