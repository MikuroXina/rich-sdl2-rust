use crate::geo::Rect;
use crate::texture::Texture;
use crate::{bind, Sdl};

use super::Renderer;

pub trait PasteExt {
    fn paste(&self, texture: Texture, target_area: Option<Rect>);
}

impl PasteExt for Renderer<'_> {
    fn paste(&self, texture: Texture, target_area: Option<Rect>) {
        let ret = unsafe {
            bind::SDL_RenderCopy(
                self.as_ptr(),
                texture.as_ptr(),
                texture
                    .clip()
                    .clone()
                    .map_or(std::ptr::null(), |rect| &rect.into() as *const _),
                target_area.map_or(std::ptr::null(), |rect| &rect.into() as *const _),
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Pasting texture to renderer");
        }
    }
}
