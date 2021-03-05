use std::ptr::NonNull;

use crate::color::pixel::palette::Palette;
use crate::color::pixel::Pixel;
use crate::color::{BlendMode, Rgb};
use crate::geo::{Point, Rect};
use crate::{bind, Sdl};

pub mod alpha;
pub mod blend;
pub mod bmp;
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

    fn fill_rect(&self, area: Option<Rect>, color: Pixel) {
        let raw_rect = area.map(|rect| rect.into());
        unsafe {
            let ret = bind::SDL_FillRect(
                self.as_ptr().as_ptr(),
                raw_rect.map_or(std::ptr::null(), |raw| &raw as *const _),
                color.as_u32(),
            );
            if ret != 0 {
                Sdl::error_then_panic("Surface filling with rectangle")
            }
        }
    }

    fn fill_rects(&self, areas: impl IntoIterator<Item = Rect>, color: Pixel) {
        let raw_rects: Vec<_> = areas.into_iter().map(|area| area.into()).collect();
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

    fn set_palette(&self, palette: &Palette) {
        let ret = unsafe { bind::SDL_SetSurfacePalette(self.as_ptr().as_ptr(), palette.as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Surface setting palette");
        }
    }

    fn copy_to(&self, src_area: Rect, dst: &Self, dst_pos: Point) {
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
                &src_rect as *const _,
                dst.as_ptr().as_ptr(),
                &mut dst_rect as *mut _,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Surface copying to another");
        }
    }
}
