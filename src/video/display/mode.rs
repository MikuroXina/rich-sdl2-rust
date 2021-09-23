use crate::color::pixel::kind::PixelFormatKind;
use crate::{bind, EnumInt};

/// A mode of the display.
#[derive(Debug, Clone)]
pub struct Mode {
    /// A pixel format of the display.
    pub pixel_format: PixelFormatKind,
    /// A width of the display.
    pub width: u32,
    /// A height of the display.
    pub height: u32,
    /// A refresh rate of the display.
    pub refresh_rate: u32,
}

impl Mode {
    pub(super) fn new(mode: bind::SDL_DisplayMode) -> Self {
        Self {
            pixel_format: PixelFormatKind::from_raw(mode.format as EnumInt),
            width: mode.w as u32,
            height: mode.h as u32,
            refresh_rate: mode.refresh_rate as u32,
        }
    }
}
