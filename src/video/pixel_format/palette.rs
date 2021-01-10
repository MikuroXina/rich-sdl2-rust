use std::os::raw::c_int;
use std::ptr::NonNull;

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
}

impl Drop for Palette {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreePalette(self.palette.as_ptr()) }
    }
}
