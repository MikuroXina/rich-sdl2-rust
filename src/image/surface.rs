//! Image surface to load some image format to [`Surface`].

use crate::{
    bind,
    surface::{RawSurface, Surface},
    Result, Sdl, SdlError,
};
use static_assertions::assert_not_impl_all;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    os::raw::c_char,
    ptr::NonNull,
};

use super::Img;

/// An image surface for the loaded picture.
pub struct ImgSurface<'img> {
    surface: NonNull<RawSurface>,
    _phantom: PhantomData<&'img ()>,
}

assert_not_impl_all!(ImgSurface: Send, Sync);

impl<'img> ImgSurface<'img> {
    /// Constructs a new image surface from the file. The format will automatically determined if `file_type` is `None`. `file_type` is case-insensitive and can be one of these:
    ///
    /// - `"TGA"`
    /// - `"CUR"`
    /// - `"ICO"`
    /// - `"BMP"`
    /// - `"GIF"`
    /// - `"JPG"`
    /// - `"LBM"`
    /// - `"PCX"`
    /// - `"PNG"`
    /// - `"PNM"`
    /// - `"SVG"`
    /// - `"TIF"`
    /// - `"XCF"`
    /// - `"XPM"`
    /// - `"XV"`
    /// - `"WEBP"`
    ///
    /// # Panics
    ///
    /// Panics if `file_name` or `file_type` is an empty string.
    pub fn new(_img: &'img Img, file_name: &str, file_type: Option<&str>) -> Result<Self> {
        let file_name_cstr = CString::new(file_name).expect("file_name mus not be empty");
        let mode = CStr::from_bytes_with_nul(b"rb\0").unwrap();
        let fp = unsafe { bind::SDL_RWFromFile(file_name_cstr.as_ptr(), mode.as_ptr()) };
        if fp.is_null() {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        let file_type_cstr = file_type
            .map(|file_type| CString::new(file_type).expect("file_name must not be empty"));
        let ptr = unsafe {
            bind::IMG_LoadTyped_RW(
                fp,
                1,
                file_type_cstr.map_or(std::ptr::null(), |cstr| cstr.as_ptr()),
            )
        };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                surface: NonNull::new(ptr.cast()).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Constructs a new image surface from XPM format str slice.
    pub fn from_xpm(_img: &'img Img, xpm: &[&str]) -> Result<Self> {
        let xpm: Vec<_> = xpm
            .iter()
            .map(|&s| CString::new(s).expect("xpm fragment must not be empty"))
            .collect();
        // SAFETY: the reason why casting to mutable pointer is the API receives C string literal array `char **` for compatibility.
        let xpm_ptr: Vec<_> = xpm.iter().map(|s| s.as_ptr() as *mut c_char).collect();
        let ptr = unsafe { bind::IMG_ReadXPMFromArray(xpm_ptr.as_ptr() as *mut _) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                surface: NonNull::new(ptr.cast()).unwrap(),
                _phantom: PhantomData,
            })
        }
    }
}

impl Surface for ImgSurface<'_> {
    fn as_ptr(&self) -> NonNull<RawSurface> {
        self.surface
    }
}

impl Drop for ImgSurface<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.surface.as_ptr().cast()) }
    }
}
