//! Glyph metrics.

use std::marker::PhantomData;

use super::Font;
use crate::{
    bind,
    geo::{Point, Rect, Size},
};

/// A glyph in a font.
pub struct Glyph<'font> {
    rect: Rect,
    advance: i32,
    _phantom: PhantomData<&'font Font<'font>>,
}

impl<'font> Glyph<'font> {
    pub(super) fn new(font: &'font Font<'font>, ch: char) -> Option<Self> {
        let provided = unsafe { bind::TTF_GlyphIsProvided32(font.ptr.as_ptr(), ch as u32) != 0 };
        if !provided {
            return None;
        }
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        let mut advance = 0;
        let ret = unsafe {
            bind::TTF_GlyphMetrics32(
                font.ptr.as_ptr(),
                ch as u32,
                &mut min_x as *mut _,
                &mut max_x as *mut _,
                &mut min_y as *mut _,
                &mut max_y as *mut _,
                &mut advance as *mut _,
            )
        };
        if ret != 0 {
            return None;
        }
        let rect = Rect {
            up_left: Point { x: min_x, y: max_y },
            size: Size {
                width: (max_x - min_x) as u32,
                height: (max_y - min_y) as u32,
            },
        };
        Some(Self {
            rect,
            advance,
            _phantom: PhantomData,
        })
    }

    /// Returns the geometry box of the glyph.
    pub fn rect(&self) -> Rect {
        self.rect
    }

    /// Returns the advance, the offset to the next character in pixels.
    pub fn advance(&self) -> i32 {
        self.advance
    }
}
