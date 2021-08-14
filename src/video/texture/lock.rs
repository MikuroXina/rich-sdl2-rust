use std::ffi::c_void;
use std::ptr::NonNull;

use crate::geo::{Rect, Size};
use crate::{bind, Sdl};

use super::Texture;

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
            .finish()
    }
}

impl<'texture> Lock<'texture> {
    pub(super) fn new(texture: &'texture mut Texture<'texture>, area: Option<Rect>) -> Self {
        let (mut pixels, mut pitch) = (std::ptr::null_mut(), 0);
        let ret = unsafe {
            bind::SDL_LockTexture(
                texture.as_ptr(),
                area.map_or(std::ptr::null(), |area| &area.into() as *const _),
                &mut pixels as *mut _,
                &mut pitch as *mut _,
            )
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

    pub fn as_bytes(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.pixels.as_ptr().cast(), self.len) }
    }
}

impl Drop for Lock<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_UnlockTexture(self.texture.as_ptr()) }
    }
}
