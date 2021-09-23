//! Color managements for graphics.

use crate::bind;

pub mod pixel;

/// A RGB color structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb {
    /// A red component in RGB.
    pub r: u8,
    /// A green component in RGB.
    pub g: u8,
    /// A blue component in RGB.
    pub b: u8,
}

impl From<u32> for Rgb {
    fn from(color_code: u32) -> Self {
        Self {
            r: (color_code >> 16) as u8,
            g: (color_code >> 8) as u8,
            b: (color_code) as u8,
        }
    }
}

/// A RGBA color structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgba {
    /// A red component in RGB.
    pub r: u8,
    /// A green component in RGB.
    pub g: u8,
    /// A blue component in RGB.
    pub b: u8,
    /// An alpha component in RGB.
    pub a: u8,
}

impl From<u32> for Rgba {
    fn from(color_code: u32) -> Self {
        Self {
            r: (color_code >> 24) as u8,
            g: (color_code >> 16) as u8,
            b: (color_code >> 8) as u8,
            a: (color_code) as u8,
        }
    }
}

impl From<Rgba> for bind::SDL_Color {
    fn from(Rgba { r, g, b, a }: Rgba) -> Self {
        Self { r, g, b, a }
    }
}

/// A mode for blending colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Not blend, overwrites a color by the another one.
    None,
    /// Blending colors by their alpha values.
    AlphaBlend,
    /// Blending colors by addition.
    Add,
    /// Blending colors by multiplication.
    Mul,
}

impl From<bind::SDL_BlendMode> for BlendMode {
    fn from(raw: bind::SDL_BlendMode) -> Self {
        use BlendMode::*;
        match raw {
            bind::SDL_BLENDMODE_BLEND => AlphaBlend,
            bind::SDL_BLENDMODE_ADD => Add,
            bind::SDL_BLENDMODE_MOD => Mul,
            _ => None,
        }
    }
}

impl From<BlendMode> for bind::SDL_BlendMode {
    fn from(raw: BlendMode) -> Self {
        use BlendMode::*;
        match raw {
            AlphaBlend => bind::SDL_BLENDMODE_BLEND,
            Add => bind::SDL_BLENDMODE_ADD,
            Mul => bind::SDL_BLENDMODE_MOD,
            None => bind::SDL_BLENDMODE_NONE,
        }
    }
}
