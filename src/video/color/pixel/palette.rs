use std::os::raw::c_int;
use std::ptr::NonNull;

use crate::color::Rgba;
use crate::{bind, Result, Sdl, SdlError};

pub struct Palette {
    palette: NonNull<bind::SDL_Palette>,
}

impl Palette {
    pub fn new(num_colors: u32) -> Result<Self> {
        NonNull::new(unsafe { bind::SDL_AllocPalette(num_colors as c_int) }).map_or_else(
            || {
                let msg = Sdl::error();
                Err(if msg == "Out of memory" {
                    SdlError::OutOfMemory
                } else {
                    SdlError::Others { msg }
                })
            },
            |palette| Ok(Self { palette }),
        )
    }

    pub(super) fn as_ptr(&self) -> *mut bind::SDL_Palette {
        self.palette.as_ptr()
    }

    pub fn set_palette(&self, colors: impl IntoIterator<Item = Rgba>) {
        let colors: Vec<_> = colors.into_iter().map(|c| c.into()).collect();
        let ret = unsafe {
            bind::SDL_SetPaletteColors(self.palette.as_ptr(), colors.as_ptr(), 0, colors.len() as _)
        };
        if ret != 0 {
            Sdl::error_then_panic("Setting palette colors");
        }
    }
}

impl Drop for Palette {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreePalette(self.palette.as_ptr()) }
    }
}
