//! Font data managers.

use std::{ffi::CString, marker::PhantomData, ptr::NonNull};

use crate::{bind, geo::Size, ttf::Ttf, Result, Sdl, SdlError};

pub use attribute::*;
pub use metric::*;
pub use render::*;
pub use setting::*;
pub use style::*;

use self::glyph::Glyph;

mod attribute;
pub mod glyph;
mod metric;
mod render;
mod setting;
mod style;

/// A pixel density per inch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dpi {
    /// A horizontal DPI.
    pub horizontal: u32,
    /// A vertical DPI.
    pub vertical: u32,
}

/// A font data structure.
pub struct Font<'ttf> {
    ptr: NonNull<bind::TTF_Font>,
    _phantom: PhantomData<&'ttf Ttf>,
}

impl<'ttf> Font<'ttf> {
    /// Constructs a font data from file name and point size.
    /// The font face in the font file can be selected by an optional index, which will be `0` if `None`.
    ///
    /// # Panics
    ///
    /// Panics if `file_name` is empty.
    pub fn new(_ttf: &'ttf Ttf, file_name: &str, point: u32, index: Option<usize>) -> Result<Self> {
        let file_name_cstr = CString::new(file_name).expect("file name must not be empty");
        let ptr = unsafe {
            bind::TTF_OpenFontIndex(file_name_cstr.as_ptr(), point as _, index.unwrap_or(0) as _)
        };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the glyph of the font if exists.
    pub fn glyph(&self, ch: char) -> Option<Glyph> {
        Glyph::new(self, ch)
    }

    /// Calculates the rendered size of the text, or `Err` on failure. The height will be same as [`MetricExt::height`].
    pub fn rendered_size(&self, text: &str) -> Result<Size> {
        let cstr = CString::new(text).unwrap_or_default();
        let mut width = 0;
        let mut height = 0;
        let ret = unsafe {
            bind::TTF_SizeUTF8(
                self.ptr.as_ptr(),
                cstr.as_ptr(),
                &mut width as *mut _,
                &mut height as *mut _,
            )
        };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Size {
                width: width as _,
                height: height as _,
            })
        }
    }

    /// Returns the rendered width in pixels and the numbers of characters until reaching `expected_width`. `Err` will be returned on out of memory.
    pub fn rendered_width(&self, text: &str, expected_width: u32) -> Result<(u32, usize)> {
        let cstr = CString::new(text).unwrap_or_default();
        let mut actual_width = 0;
        let mut count = 0;
        let ret = unsafe {
            bind::TTF_MeasureUTF8(
                self.ptr.as_ptr(),
                cstr.as_ptr(),
                expected_width as _,
                &mut actual_width as *mut _,
                &mut count as *mut _,
            )
        };
        if ret != 0 {
            Err(SdlError::OutOfMemory)
        } else {
            Ok((actual_width as _, count as _))
        }
    }
}

impl Drop for Font<'_> {
    fn drop(&mut self) {
        unsafe { bind::TTF_CloseFont(self.ptr.as_ptr()) }
    }
}
