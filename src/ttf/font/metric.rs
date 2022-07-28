use crate::bind;

use super::Font;

/// An extension for getters of the font metrics.
pub trait MetricExt {
    /// Returns the maximum height of the glyph in the font, in pixels.
    fn height(&self) -> u32;
    /// Returns the maximum ascent, the hight above baseline of the glyph in the font, in pixels.
    fn ascent(&self) -> u32;
    /// Returns the maximum ascent, the hight below baseline of the glyph in the font, in pixels.
    fn descent(&self) -> u32;
    /// Returns the line skip height of the font, the recommended height of a rendered line of text, in pixels. It is usually larger than [`MetricExt::height()`].
    fn line_skip(&self) -> u32;
}

impl MetricExt for Font<'_> {
    fn height(&self) -> u32 {
        unsafe { bind::TTF_FontHeight(self.ptr.as_ptr()) as _ }
    }

    fn ascent(&self) -> u32 {
        unsafe { bind::TTF_FontAscent(self.ptr.as_ptr()) as _ }
    }

    fn descent(&self) -> u32 {
        unsafe { bind::TTF_FontDescent(self.ptr.as_ptr()) as _ }
    }

    fn line_skip(&self) -> u32 {
        unsafe { bind::TTF_FontLineSkip(self.ptr.as_ptr()) as _ }
    }
}
