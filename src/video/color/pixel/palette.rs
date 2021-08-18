//! Palettes for a bitmap pixel format.

use std::os::raw::c_int;
use std::ptr::NonNull;

use static_assertions::assert_not_impl_all;

use crate::color::Rgba;
use crate::{bind, Result, Sdl, SdlError};

/// A controller for a palette in an indexed bitmap pixel format.
pub struct Palette {
    pub(super) palette: NonNull<bind::SDL_Palette>,
}

impl std::fmt::Debug for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Palette").finish()
    }
}

assert_not_impl_all!(Palette: Send, Sync);

impl Palette {
    /// Constructs a palette with numbers of colors.
    pub fn new(num_colors: usize) -> Result<Self> {
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

    pub(crate) fn as_ptr(&self) -> *mut bind::SDL_Palette {
        self.palette.as_ptr()
    }

    /// Returns the numbers of colors in the palette.
    pub fn num_colors(&self) -> usize {
        unsafe { self.palette.as_ref() }.ncolors as _
    }

    /// Sets colors to the palette, or panics if `colors` was too long than `num_colors` on initialized.
    ///
    /// # Panics
    ///
    /// Panics if `colors` was too long than `num_colors` on initialized.
    pub fn set_colors(&self, colors: impl IntoIterator<Item = Rgba>) {
        let colors: Vec<_> = colors.into_iter().map(|c| c.into()).collect();
        assert!(colors.len() <= self.num_colors());
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
