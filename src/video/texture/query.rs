use crate::color::pixel::kind::PixelFormatKind;
use crate::geo::Size;
use crate::{bind, Sdl};

use super::{Texture, TextureAccess};

pub trait QueryExt {
    fn format(&self) -> PixelFormatKind;
    fn access(&self) -> TextureAccess;
    fn size(&self) -> Size;
}

impl QueryExt for Texture<'_> {
    fn format(&self) -> PixelFormatKind {
        use std::ptr::null_mut;
        let mut raw_format = 0;
        let ret = unsafe {
            bind::SDL_QueryTexture(
                self.as_ptr(),
                &mut raw_format as *mut _,
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture format");
        }
        raw_format.into()
    }

    fn access(&self) -> TextureAccess {
        use std::ptr::null_mut;
        let mut access = 0;
        let ret = unsafe {
            bind::SDL_QueryTexture(
                self.as_ptr(),
                null_mut(),
                &mut access as *mut _,
                null_mut(),
                null_mut(),
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture access");
        }
        TextureAccess::from_raw(access as u32)
    }

    fn size(&self) -> Size {
        use std::ptr::null_mut;
        let (mut w, mut h) = (0, 0);
        let ret = unsafe {
            bind::SDL_QueryTexture(
                self.as_ptr(),
                null_mut(),
                null_mut(),
                &mut w as *mut _,
                &mut h as *mut _,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture size");
        }
        Size {
            width: w as u32,
            height: h as u32,
        }
    }
}
