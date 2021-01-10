use crate::color::Rgb;
use crate::geo::Rect;
use crate::video::geo::{Line, Point};
use crate::{bind, Sdl};

use super::Renderer;

pub enum BlendMode {
    None,
    AlphaBlend,
    Add,
    Mul,
}

impl From<bind::SDL_BlendMode> for BlendMode {
    fn from(raw: bind::SDL_BlendMode) -> Self {
        use BlendMode::*;
        match raw {
            bind::SDL_BlendMode_SDL_BLENDMODE_BLEND => AlphaBlend,
            bind::SDL_BlendMode_SDL_BLENDMODE_ADD => Add,
            bind::SDL_BlendMode_SDL_BLENDMODE_MOD => Mul,
            _ => None,
        }
    }
}

impl From<BlendMode> for bind::SDL_BlendMode {
    fn from(raw: BlendMode) -> Self {
        use BlendMode::*;
        match raw {
            AlphaBlend => bind::SDL_BlendMode_SDL_BLENDMODE_BLEND,
            Add => bind::SDL_BlendMode_SDL_BLENDMODE_ADD,
            Mul => bind::SDL_BlendMode_SDL_BLENDMODE_MOD,
            None => bind::SDL_BlendMode_SDL_BLENDMODE_NONE,
        }
    }
}

pub struct Pen<'renderer> {
    renderer: &'renderer Renderer<'renderer>,
}

impl<'renderer> Pen<'renderer> {
    pub fn new(renderer: &'renderer Renderer) -> Self {
        Self { renderer }
    }

    pub fn set_color(&self, Rgb { r, g, b }: Rgb) {
        let ret = unsafe { bind::SDL_SetRenderDrawColor(self.renderer.as_ptr(), r, g, b, 255) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen color")
        }
    }

    pub fn clear(&self) {
        let ret = unsafe { bind::SDL_RenderClear(self.renderer.as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen clear")
        }
    }

    pub fn blend_mode(&self) -> BlendMode {
        let mut raw = 0;
        let ret =
            unsafe { bind::SDL_GetRenderDrawBlendMode(self.renderer.as_ptr(), &mut raw as *mut _) };
        if ret != 0 {
            Sdl::error_then_panic("Getting renderer blend mode")
        }
        raw.into()
    }

    pub fn set_blend_mode(&self, mode: BlendMode) {
        let ret = unsafe { bind::SDL_SetRenderDrawBlendMode(self.renderer.as_ptr(), mode.into()) };
        if ret != 0 {
            Sdl::error_then_panic("Getting renderer blend mode")
        }
    }

    pub fn line(&self, line: Line) {
        let ret = unsafe {
            bind::SDL_RenderDrawLine(
                self.renderer.as_ptr(),
                line.start.x,
                line.start.y,
                line.end.x,
                line.end.y,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen line")
        }
    }

    pub fn lines(&self, points: impl IntoIterator<Item = Point>) {
        let points: Vec<_> = points.into_iter().map(|p| p.into()).collect();
        let ret = unsafe {
            bind::SDL_RenderDrawLines(self.renderer.as_ptr(), points.as_ptr(), points.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen lines")
        }
    }

    pub fn point(&self, point: Point) {
        let ret = unsafe { bind::SDL_RenderDrawPoint(self.renderer.as_ptr(), point.x, point.y) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen point")
        }
    }

    pub fn points(&self, points: impl IntoIterator<Item = Point>) {
        let points: Vec<_> = points.into_iter().map(|p| p.into()).collect();
        let ret = unsafe {
            bind::SDL_RenderDrawPoints(self.renderer.as_ptr(), points.as_ptr(), points.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen points")
        }
    }

    pub fn stroke_rect(&self, rect: Rect) {
        let ret =
            unsafe { bind::SDL_RenderDrawRect(self.renderer.as_ptr(), &rect.into() as *const _) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen rect")
        }
    }

    pub fn stroke_rects(&self, rects: impl IntoIterator<Item = Rect>) {
        let rects: Vec<_> = rects.into_iter().map(|r| r.into()).collect();
        let ret = unsafe {
            bind::SDL_RenderDrawRects(self.renderer.as_ptr(), rects.as_ptr(), rects.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen rects")
        }
    }

    pub fn fill_rect(&self, rect: Rect) {
        let ret =
            unsafe { bind::SDL_RenderFillRect(self.renderer.as_ptr(), &rect.into() as *const _) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen rect")
        }
    }

    pub fn fill_rects(&self, rects: impl IntoIterator<Item = Rect>) {
        let rects: Vec<_> = rects.into_iter().map(|r| r.into()).collect();
        let ret = unsafe {
            bind::SDL_RenderFillRects(self.renderer.as_ptr(), rects.as_ptr(), rects.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen rects")
        }
    }
}

impl<'renderer> Drop for Pen<'renderer> {
    fn drop(&mut self) {
        unsafe { bind::SDL_RenderPresent(self.renderer.as_ptr()) }
    }
}
