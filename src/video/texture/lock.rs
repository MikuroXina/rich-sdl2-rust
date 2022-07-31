//! Locking the texture for read/write.

use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{
    as_raw,
    geo::{Rect, Size},
};
use crate::{bind, Sdl};

use super::Texture;

/// A lock of the texture, ready to read/write as the raw pixels.
pub struct Lock<'texture> {
    texture: &'texture mut Texture<'texture>,
    pixels: NonNull<c_void>,
    len: usize,
}

impl std::fmt::Debug for Lock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lock")
            .field("texture", &self.texture)
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}

impl<'texture> Lock<'texture> {
    pub(super) fn new(texture: &'texture mut Texture<'texture>, area: Option<Rect>) -> Self {
        let area = area.map(Into::into);
        let (mut pixels, mut pitch) = (std::ptr::null_mut(), 0);
        let ret = unsafe {
            bind::SDL_LockTexture(texture.as_ptr(), as_raw(&area), &mut pixels, &mut pitch)
        };
        if ret != 0 {
            Sdl::error_then_panic("Obtaining texture lock");
        }

        use super::QueryExt;
        let Size { height, .. } = texture.size();
        Self {
            texture,
            pixels: NonNull::new(pixels).unwrap(),
            len: (pitch as u32 * height) as usize,
        }
    }

    /// Returns bytes of the pixels.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.pixels.as_ptr().cast(), self.len) }
    }

    /// Returns mutable bytes of the pixels.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.pixels.as_ptr().cast(), self.len) }
    }
}

impl Drop for Lock<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_UnlockTexture(self.texture.as_ptr()) }
    }
}
