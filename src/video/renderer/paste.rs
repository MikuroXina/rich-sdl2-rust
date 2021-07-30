use bitflags::bitflags;

use crate::geo::{Point, Rect};
use crate::texture::Texture;
use crate::{bind, Sdl};

use super::Renderer;

bitflags! {
    pub struct PasteExFlip: u32 {
        const HORIZONTAL = bind::SDL_RendererFlip_SDL_FLIP_HORIZONTAL;
        const VERTICAL = bind::SDL_RendererFlip_SDL_FLIP_VERTICAL;
        const BOTH = Self::HORIZONTAL.bits | Self::VERTICAL.bits;
    }
}

impl Default for PasteExFlip {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Default)]
pub struct PasteExOption {
    pub target_area: Option<Rect>,
    pub rotation_degrees: f64,
    pub center: Option<Point>,
    pub flip: PasteExFlip,
}

pub trait PasteExt {
    fn paste(&self, texture: Texture, target_area: Option<Rect>);
    fn paste_ex(&self, texture: Texture, options: PasteExOption);
}

impl PasteExt for Renderer<'_> {
    fn paste(&self, texture: Texture, target_area: Option<Rect>) {
        let ret = unsafe {
            bind::SDL_RenderCopy(
                self.as_ptr(),
                texture.as_ptr(),
                texture
                    .clip()
                    .map_or(std::ptr::null(), |rect| &rect.into() as *const _),
                target_area.map_or(std::ptr::null(), |rect| &rect.into() as *const _),
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Pasting texture to renderer");
        }
    }

    fn paste_ex(
        &self,
        texture: Texture,
        PasteExOption {
            target_area,
            rotation_degrees,
            center,
            flip,
        }: PasteExOption,
    ) {
        let ret = unsafe {
            bind::SDL_RenderCopyEx(
                self.as_ptr(),
                texture.as_ptr(),
                texture
                    .clip()
                    .map_or(std::ptr::null(), |rect| &rect.into() as *const _),
                target_area.map_or(std::ptr::null(), |rect| &rect.into() as *const _),
                rotation_degrees,
                center.map_or(std::ptr::null(), |p| &p.into() as *const _),
                flip.bits,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Pasting texture to renderer ex");
        }
    }
}
