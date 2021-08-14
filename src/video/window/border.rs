use crate::{bind, Result};

use super::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderWidths {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

pub trait BorderExt {
    fn border_widths(&self) -> Result<BorderWidths>;
}

impl BorderExt for Window<'_> {
    fn border_widths(&self) -> Result<BorderWidths> {
        let (mut top, mut left, mut bottom, mut right) = (0, 0, 0, 0);
        let ret = unsafe {
            bind::SDL_GetWindowBordersSize(
                self.as_ptr(),
                &mut top as *mut _,
                &mut left as *mut _,
                &mut bottom as *mut _,
                &mut right as *mut _,
            )
        };
        if ret != 0 {
            return Err(crate::SdlError::UnsupportedFeature);
        }
        Ok(BorderWidths {
            top: top as u32,
            right: right as u32,
            bottom: bottom as u32,
            left: left as u32,
        })
    }
}
