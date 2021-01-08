use std::mem::MaybeUninit;

use crate::bind;
use crate::pixel_format::kind::PixelFormatKind;

use super::Display;

pub struct Mode {
    pub index: i32,
    pub pixel_format: PixelFormatKind,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

impl Mode {
    pub(super) fn new(index: i32, disp: &Display) -> Self {
        let mut raw = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetDisplayMode(disp.index, index, raw.as_mut_ptr()) };
        debug_assert!(ret != 0);
        let mode = unsafe { raw.assume_init() };
        Self {
            index,
            pixel_format: mode.format.into(),
            width: mode.w as u32,
            height: mode.h as u32,
            refresh_rate: mode.refresh_rate as u32,
        }
    }
}
