use crate::bind;
use crate::pixel_format::kind::PixelFormatKind;

pub struct Mode {
    pub pixel_format: PixelFormatKind,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

impl Mode {
    pub(super) fn new(mode: bind::SDL_DisplayMode) -> Self {
        Self {
            pixel_format: mode.format.into(),
            width: mode.w as u32,
            height: mode.h as u32,
            refresh_rate: mode.refresh_rate as u32,
        }
    }
}
