use crate::color::pixel::kind::PixelFormatKind;
use crate::geo::Size;
use crate::{bind, EnumInt, Sdl};

use super::{Texture, TextureAccess};

/// An extension for [`Texture`] to query the texture information.
pub trait QueryExt {
    /// Returns the format of the texture.
    fn format(&self) -> PixelFormatKind;
    /// Returns the access of the texture.
    fn access(&self) -> TextureAccess;
    /// Returns the size of the texture.
    fn size(&self) -> Size;
}

impl QueryExt for Texture<'_> {
    fn format(&self) -> PixelFormatKind {
        use std::ptr::null_mut;
        let mut raw_format = 0u32;
        let ret = unsafe {
            bind::SDL_QueryTexture(
                self.as_ptr(),
                &mut raw_format,
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture format");
        }
        PixelFormatKind::from_raw(raw_format as EnumInt)
    }

    fn access(&self) -> TextureAccess {
        use std::ptr::null_mut;
        let mut access = 0;
        let ret = unsafe {
            bind::SDL_QueryTexture(
                self.as_ptr(),
                null_mut(),
                &mut access,
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
            bind::SDL_QueryTexture(self.as_ptr(), null_mut(), null_mut(), &mut w, &mut h)
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
