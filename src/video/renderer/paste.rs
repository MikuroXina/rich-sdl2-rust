#![allow(clippy::unnecessary_cast)]

use bitflags::bitflags;

use crate::texture::Texture;
use crate::{
    as_raw,
    geo::{Point, Rect},
};
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

/// A paster controls pasting from a texture.
///
/// This will render when be dropped. So you should re-create on every render.
#[derive(Debug)]
pub struct Paster<'renderer> {
    renderer: &'renderer Renderer<'renderer>,
}

impl<'renderer> Drop for Paster<'renderer> {
    fn drop(&mut self) {
        unsafe { bind::SDL_RenderPresent(self.renderer.as_ptr()) }
    }
}

impl<'renderer> Paster<'renderer> {
    /// Constructs a pen from the renderer [`Renderer`].
    #[must_use]
    pub fn new(renderer: &'renderer Renderer) -> Self {
        Self { renderer }
    }

    /// Returns the renderer that the pen is drawing.
    #[must_use]
    pub fn renderer(&self) -> &Renderer {
        self.renderer
    }
}

impl Paster<'_> {
    /// Pastes the texture into `target_area`, or whole if `None`.
    pub fn paste(&self, texture: &Texture, target_area: Option<Rect>) {
        let src = texture.clip().map(Into::into);
        let dst = target_area.map(Into::into);
        let ret = unsafe {
            bind::SDL_RenderCopy(
                self.renderer.as_ptr(),
                texture.as_ptr(),
                as_raw(&src),
                as_raw(&dst),
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Pasting texture to renderer");
        }
    }

    /// Pastes the texture with options [`PasteExOption`].
    pub fn paste_ex(
        &self,
        texture: &Texture,
        PasteExOption {
            target_area,
            rotation_degrees,
            center,
            flip,
        }: PasteExOption,
    ) {
        let src = texture.clip().map(Into::into);
        let dst = target_area.map(Into::into);
        let center = center.map(Into::into);
        let ret = unsafe {
            bind::SDL_RenderCopyEx(
                self.renderer.as_ptr(),
                texture.as_ptr(),
                as_raw(&src),
                as_raw(&dst),
                rotation_degrees,
                as_raw(&center),
                flip.bits as EnumInt,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Pasting texture to renderer ex");
        }
    }
}
