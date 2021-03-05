use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::{bind, Sdl};

use super::Surface;

pub struct Bmp {
    ptr: NonNull<bind::SDL_Surface>,
}

impl Bmp {
    pub fn new(file_name: &str) -> Self {
        let c_str = CString::new(file_name).expect("must be a valid string");
        let read_binary_mode = CStr::from_bytes_with_nul(b"rb\0").unwrap();
        let ptr = NonNull::new(unsafe {
            bind::SDL_LoadBMP_RW(
                bind::SDL_RWFromFile(c_str.as_ptr(), read_binary_mode.as_ptr()),
                1,
            )
        })
        .unwrap_or_else(|| Sdl::error_then_panic("Surface from BMP file"));
        Self { ptr }
    }
}

impl Surface for Bmp {
    fn as_ptr(&self) -> NonNull<bind::SDL_Surface> {
        self.ptr
    }
}

impl Drop for Bmp {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.ptr.as_ptr()) }
    }
}
