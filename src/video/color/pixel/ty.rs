//! Types for a pixel format.

use crate::{bind, EnumInt};

/// A type in a bitmap pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitmapPixelType {
    /// An 1 bit index of the palette with 2 colors.
    Index1,
    /// A 4 bits index of the palette with 16 colors.
    Index4,
    /// An 8 bits index of the palette with 256 colors.
    Index8,
}

impl BitmapPixelType {
    #[allow(clippy::unnecessary_cast)]
    pub(super) fn as_raw(self) -> u32 {
        (match self {
            BitmapPixelType::Index1 => bind::SDL_PIXELTYPE_INDEX1,
            BitmapPixelType::Index4 => bind::SDL_PIXELTYPE_INDEX4,
            BitmapPixelType::Index8 => bind::SDL_PIXELTYPE_INDEX8,
        }) as u32
    }

    pub(super) fn bits_per_pixel(self) -> u32 {
        match self {
            BitmapPixelType::Index1 => 1,
            BitmapPixelType::Index4 => 4,
            BitmapPixelType::Index8 => 8,
        }
    }

    pub(super) fn bytes_per_pixel(self) -> u32 {
        if let BitmapPixelType::Index8 = self {
            1
        } else {
            0
        }
    }
}

/// A type in a packed pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackedPixelType {
    /// An unsigned 8 bits.
    _8,
    /// An unsigned 16 bits.
    _16,
    /// An unsigned 32 bits.
    _32,
}

impl PackedPixelType {
    #[allow(clippy::unnecessary_cast)]
    pub(super) fn as_raw(self) -> u32 {
        (match self {
            PackedPixelType::_8 => bind::SDL_PIXELTYPE_PACKED8,
            PackedPixelType::_16 => bind::SDL_PIXELTYPE_PACKED16,
            PackedPixelType::_32 => bind::SDL_PIXELTYPE_PACKED32,
        }) as u32
    }

    pub(super) fn bytes_per_pixel(self) -> u32 {
        match self {
            PackedPixelType::_8 => 1,
            PackedPixelType::_16 => 2,
            PackedPixelType::_32 => 4,
        }
    }
}

/// A type in a pixel array format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ArrayPixelType {
    /// A type of array is `u8`.
    U8,
    /// A type of array is `u16`.
    U16,
    /// A type of array is `u32`.
    U32,
    /// A type of array is `f16` (note that not supported by first party).
    F16,
    /// A type of array is `f32`.
    F32,
}

impl ArrayPixelType {
    #[allow(clippy::unnecessary_cast)]
    pub(super) fn as_raw(self) -> u32 {
        (match self {
            ArrayPixelType::U8 => bind::SDL_PIXELTYPE_ARRAYU8,
            ArrayPixelType::U16 => bind::SDL_PIXELTYPE_ARRAYU16,
            ArrayPixelType::U32 => bind::SDL_PIXELTYPE_ARRAYU32,
            ArrayPixelType::F16 => bind::SDL_PIXELTYPE_ARRAYF16,
            ArrayPixelType::F32 => bind::SDL_PIXELTYPE_ARRAYF32,
        }) as u32
    }
}
