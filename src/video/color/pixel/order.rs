//! Pixel components orders for a pixel format.

use crate::bind;

/// A pixel order in a bitmap pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitmapPixelOrder {
    /// An order in little endian.
    _4321,
    /// An order in big endian.
    _1234,
}

impl BitmapPixelOrder {
    #[allow(clippy::unnecessary_cast)]
    pub(super) fn as_raw(self) -> u32 {
        (match self {
            BitmapPixelOrder::_4321 => bind::SDL_BITMAPORDER_4321,
            BitmapPixelOrder::_1234 => bind::SDL_BITMAPORDER_1234,
        }) as u32
    }
}

impl From<bind::SDL_PixelFormatEnum> for BitmapPixelOrder {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        match (raw >> 20) & 0xf {
            bind::SDL_BITMAPORDER_4321 => BitmapPixelOrder::_4321,
            bind::SDL_BITMAPORDER_1234 => BitmapPixelOrder::_1234,
            _ => unreachable!(),
        }
    }
}

/// A pixel order in a packed pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackedPixelOrder {
    /// An order of 3 components is right aligned RGB.
    Xrgb,
    /// An order of 3 components is left aligned RGB.
    Rgbx,
    /// An order of 4 components is ARGB.
    Argb,
    /// An order of 4 components is RGBA.
    Rgba,
    /// An order of 3 components is right aligned BGR.
    Xbgr,
    /// An order of 3 components is left aligned BGR.
    Bgrx,
    /// An order of 4 components is ABGR.
    Abgr,
    /// An order of 4 components is BGRA.
    Bgra,
}

impl PackedPixelOrder {
    #[allow(clippy::unnecessary_cast)]
    pub(super) fn as_raw(self) -> u32 {
        (match self {
            PackedPixelOrder::Xrgb => bind::SDL_PACKEDORDER_XRGB,
            PackedPixelOrder::Rgbx => bind::SDL_PACKEDORDER_RGBX,
            PackedPixelOrder::Argb => bind::SDL_PACKEDORDER_ARGB,
            PackedPixelOrder::Rgba => bind::SDL_PACKEDORDER_RGBA,
            PackedPixelOrder::Xbgr => bind::SDL_PACKEDORDER_XBGR,
            PackedPixelOrder::Bgrx => bind::SDL_PACKEDORDER_BGRX,
            PackedPixelOrder::Abgr => bind::SDL_PACKEDORDER_ABGR,
            PackedPixelOrder::Bgra => bind::SDL_PACKEDORDER_BGRA,
        }) as u32
    }
}

impl From<bind::SDL_PixelFormatEnum> for PackedPixelOrder {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        match (raw >> 20) & 0xf {
            bind::SDL_PACKEDORDER_XRGB => PackedPixelOrder::Xrgb,
            bind::SDL_PACKEDORDER_RGBX => PackedPixelOrder::Rgbx,
            bind::SDL_PACKEDORDER_ARGB => PackedPixelOrder::Argb,
            bind::SDL_PACKEDORDER_RGBA => PackedPixelOrder::Rgba,
            bind::SDL_PACKEDORDER_XBGR => PackedPixelOrder::Xbgr,
            bind::SDL_PACKEDORDER_BGRX => PackedPixelOrder::Bgrx,
            bind::SDL_PACKEDORDER_ABGR => PackedPixelOrder::Abgr,
            bind::SDL_PACKEDORDER_BGRA => PackedPixelOrder::Bgra,
            _ => unreachable!(),
        }
    }
}

/// A pixel byte order from low byte to high byte for a array pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayPixelOrder {
    /// An order stored in RGB.
    Rgb,
    /// An order stored in RGBA.
    Rgba,
    /// An order stored in ARGB.
    Argb,
    /// An order stored in BGR.
    Bgr,
    /// An order stored in BGRA.
    Bgra,
    /// An order stored in ABGR.
    Abgr,
}

impl ArrayPixelOrder {
    #[allow(clippy::unnecessary_cast)]
    pub(super) fn as_raw(self) -> u32 {
        (match self {
            ArrayPixelOrder::Rgb => bind::SDL_ARRAYORDER_RGB,
            ArrayPixelOrder::Rgba => bind::SDL_ARRAYORDER_RGBA,
            ArrayPixelOrder::Argb => bind::SDL_ARRAYORDER_ARGB,
            ArrayPixelOrder::Bgr => bind::SDL_ARRAYORDER_BGR,
            ArrayPixelOrder::Bgra => bind::SDL_ARRAYORDER_BGRA,
            ArrayPixelOrder::Abgr => bind::SDL_ARRAYORDER_ABGR,
        }) as u32
    }
}

impl From<bind::SDL_PixelFormatEnum> for ArrayPixelOrder {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        match (raw >> 20) & 0xf {
            bind::SDL_ARRAYORDER_RGB => ArrayPixelOrder::Rgb,
            bind::SDL_ARRAYORDER_RGBA => ArrayPixelOrder::Rgba,
            bind::SDL_ARRAYORDER_ARGB => ArrayPixelOrder::Argb,
            bind::SDL_ARRAYORDER_BGR => ArrayPixelOrder::Bgr,
            bind::SDL_ARRAYORDER_BGRA => ArrayPixelOrder::Bgra,
            bind::SDL_ARRAYORDER_ABGR => ArrayPixelOrder::Abgr,
            _ => unreachable!(),
        }
    }
}
