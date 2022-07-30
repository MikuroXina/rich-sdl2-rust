//! Bitmap texture as a [`Surface`].

use static_assertions::assert_not_impl_all;
use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::{bind, Result, Sdl, SdlError};

use super::{RawSurface, Surface};

/// A bitmap texture as a [`Surface`].
pub struct Bmp {
    ptr: NonNull<RawSurface>,
}

impl std::fmt::Debug for Bmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bmp").finish_non_exhaustive()
    }
}

assert_not_impl_all!(Bmp: Send, Sync);

impl Bmp {
    /// Constructs from the bitmap file name.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to open the file, uses an unknown data format, or is corrupt.
    ///
    /// # Panics
    ///
    /// Panics if `file_name` contains a null character.
    pub fn new(file_name: &str) -> Result<Self> {
        let c_str = CString::new(file_name).expect("must be a valid string");
        let read_binary_mode = CStr::from_bytes_with_nul(b"rb\0").unwrap();
        let ptr = NonNull::new(unsafe {
            bind::SDL_LoadBMP_RW(
                bind::SDL_RWFromFile(c_str.as_ptr(), read_binary_mode.as_ptr()),
                1,
            )
        })
        .ok_or_else(|| SdlError::Others { msg: Sdl::error() })?;
        Ok(Self { ptr })
    }
}

impl Surface for Bmp {
    fn as_ptr(&self) -> NonNull<RawSurface> {
        self.ptr
    }
}

impl Drop for Bmp {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.ptr.as_ptr()) }
    }
}

/// An error on saving as the bitmap texture.
#[derive(Debug, Clone)]
pub struct BmpSaveError(pub String);

/// An extension for a [`Surface`] to save the image as BMP format.
pub trait BmpSaveExt {
    /// Saves the surface image as BMP format.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to save an image to the file.
    fn save_bmp(&self, file_name: &str) -> std::result::Result<(), BmpSaveError>;
}

impl<T: Surface> BmpSaveExt for T {
    fn save_bmp(&self, file_name: &str) -> std::result::Result<(), BmpSaveError> {
        let write_binary_mode = CStr::from_bytes_with_nul(b"wb\0").unwrap();
        let c_str = CString::new(file_name).expect("must be a valid string");
        let ret = unsafe {
            bind::SDL_SaveBMP_RW(
                self.as_ptr().as_ptr(),
                bind::SDL_RWFromFile(c_str.as_ptr(), write_binary_mode.as_ptr()),
                1,
            )
        };
        if ret != 0 {
            return Err(BmpSaveError(Sdl::error()));
        }
        Ok(())
    }
}
