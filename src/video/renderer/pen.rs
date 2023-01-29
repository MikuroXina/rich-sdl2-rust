//! Rendering geometries and setting colors.

use crate::color::{BlendMode, Rgb};
use crate::geo::Rect;
use crate::video::geo::{Line, Point};
use crate::{bind, Sdl};

use super::Renderer;

/// A pen controls its color and renders geometries to the renderer.
///
/// This will render when be dropped. So you should re-create on every render.
#[derive(Debug)]
pub struct Pen<'renderer> {
    renderer: &'renderer Renderer<'renderer>,
}

impl<'renderer> Pen<'renderer> {
    /// Constructs a pen from the renderer [`Renderer`].
    #[must_use]
    pub fn new(renderer: &'renderer Renderer) -> Self {
        let this = Self { renderer };
        this.set_color(Rgb { r: 0, g: 0, b: 0 });
        this.clear();
        this
    }

    /// Returns the renderer that the pen is drawing.
    #[must_use]
    pub fn renderer(&self) -> &Renderer {
        self.renderer
    }

    /// Sets the drawing color.
    pub fn set_color(&self, Rgb { r, g, b }: Rgb) {
        let ret = unsafe { bind::SDL_SetRenderDrawColor(self.renderer.as_ptr(), r, g, b, 255) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen color")
        }
    }

    /// Returns the drawing color.
    pub fn color(&self) -> Rgb {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let ret = unsafe {
            bind::SDL_GetRenderDrawColor(
                self.renderer.as_ptr(),
                &mut r,
                &mut g,
                &mut b,
                std::ptr::null_mut(),
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen color")
        }
        Rgb { r, g, b }
    }

    /// Clears all the renderer area.
    pub fn clear(&self) {
        let ret = unsafe { bind::SDL_RenderClear(self.renderer.as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen clear")
        }
    }

    /// Returns the current color blend mode.
    pub fn blend_mode(&self) -> BlendMode {
        let mut raw = 0;
        let ret = unsafe { bind::SDL_GetRenderDrawBlendMode(self.renderer.as_ptr(), &mut raw) };
        if ret != 0 {
            Sdl::error_then_panic("Getting renderer blend mode")
        }
        raw.into()
    }

    /// Sets the color blend mode.
    pub fn set_blend_mode(&self, mode: BlendMode) {
        let ret = unsafe { bind::SDL_SetRenderDrawBlendMode(self.renderer.as_ptr(), mode.into()) };
        if ret != 0 {
            Sdl::error_then_panic("Getting renderer blend mode")
        }
    }

    /// Draws the line.
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

    /// Draws the lines.
    pub fn lines(&self, points: impl IntoIterator<Item = Point>) {
        let points: Vec<_> = points.into_iter().map(Into::into).collect();
        let ret = unsafe {
            bind::SDL_RenderDrawLines(self.renderer.as_ptr(), points.as_ptr(), points.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen lines")
        }
    }

    /// Draw the point.
    pub fn point(&self, point: Point) {
        let ret = unsafe { bind::SDL_RenderDrawPoint(self.renderer.as_ptr(), point.x, point.y) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen point")
        }
    }

    /// Draw the points.
    pub fn points(&self, points: impl IntoIterator<Item = Point>) {
        let points: Vec<_> = points.into_iter().map(Into::into).collect();
        let ret = unsafe {
            bind::SDL_RenderDrawPoints(self.renderer.as_ptr(), points.as_ptr(), points.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen points")
        }
    }

    /// Draw the rectangle only lines.
    pub fn stroke_rect(&self, rect: Rect) {
        let ret = unsafe { bind::SDL_RenderDrawRect(self.renderer.as_ptr(), &rect.into()) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen rect")
        }
    }

    /// Draw the rectangles only lines.
    pub fn stroke_rects(&self, rects: impl IntoIterator<Item = Rect>) {
        let rects: Vec<_> = rects.into_iter().map(Into::into).collect();
        let ret = unsafe {
            bind::SDL_RenderDrawRects(self.renderer.as_ptr(), rects.as_ptr(), rects.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen rects")
        }
    }

    /// Draw the filled rectangle.
    pub fn fill_rect(&self, rect: Rect) {
        let ret = unsafe { bind::SDL_RenderFillRect(self.renderer.as_ptr(), &rect.into()) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen rect")
        }
    }

    /// Draw the filled rectangles.
    pub fn fill_rects(&self, rects: impl IntoIterator<Item = Rect>) {
        let rects: Vec<_> = rects.into_iter().map(Into::into).collect();
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
