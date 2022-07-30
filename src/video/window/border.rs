use crate::{bind, Result};

use super::Window;

/// A border widths for the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderWidths {
    /// A border width on the top.
    pub top: u32,
    /// A border width on the right.
    pub right: u32,
    /// A border width on the bottom.
    pub bottom: u32,
    /// A border width on the left.
    pub left: u32,
}

/// An extension for [`Window`] to query border widths.
pub trait BorderExt {
    /// Returns the border widths of the window.
    ///
    /// # Errors
    ///
    /// Returns `Err` if querying them is unsupported.
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
