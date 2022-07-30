use bitflags::bitflags;

use crate::geo::{Point, Rect};
use crate::texture::Texture;
use crate::{bind, EnumInt, Sdl};

use super::Renderer;

bitflags! {
    /// Flip mode on pasting from another texture.
    pub struct PasteExFlip: u32 {
        /// Flips horizontal.
        const HORIZONTAL = bind::SDL_FLIP_HORIZONTAL as u32;
        /// Flips vertical.
        const VERTICAL = bind::SDL_FLIP_VERTICAL as u32;
        /// Flips both horizontal and vertical.
        const BOTH = Self::HORIZONTAL.bits | Self::VERTICAL.bits;
    }
}

impl Default for PasteExFlip {
    fn default() -> Self {
        Self::empty()
    }
}

/// An option for [`PasteExt::paste_ex`].
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PasteExOption {
    /// The target area of pasting, or whole if `None`.
    pub target_area: Option<Rect>,
    /// The degrees of rotating another texture.
    pub rotation_degrees: f64,
    /// The center point of pasting.
    pub center: Option<Point>,
    /// The flip mode of pasting.
    pub flip: PasteExFlip,
}

/// An extension for [`Renderer`] to paste from another texture.
pub trait PasteExt {
    /// Pastes the texture into `target_area`, or whole if `None`.
    fn paste(&self, texture: Texture, target_area: Option<Rect>);
    /// Pastes the texture with options [`PasteExOption`].
    fn paste_ex(&self, texture: Texture, options: PasteExOption);
}

impl PasteExt for Renderer<'_> {
    fn paste(&self, texture: Texture, target_area: Option<Rect>) {
        let ret = unsafe {
            bind::SDL_RenderCopy(
                self.as_ptr(),
                texture.as_ptr(),
                texture.clip().map_or(std::ptr::null(), |rect| &rect.into()),
                target_area.map_or(std::ptr::null(), |rect| &rect.into()),
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
                texture.clip().map_or(std::ptr::null(), |rect| &rect.into()),
                target_area.map_or(std::ptr::null(), |rect| &rect.into()),
                rotation_degrees,
                center.map_or(std::ptr::null(), |p| &p.into()),
                flip.bits as EnumInt,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Pasting texture to renderer ex");
        }
    }
}
