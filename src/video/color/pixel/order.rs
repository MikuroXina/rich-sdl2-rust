//! Pixel components orders for a pixel format.

use crate::bind;

/// A pixel order in a bitmap pixel format.
#[derive(Debug, Clone)]
pub enum BitmapPixelOrder {
    /// An order in little endian.
    _4321,
    /// An order in big endian.
    _1234,
}

impl BitmapPixelOrder {
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            BitmapPixelOrder::_4321 => bind::SDL_BitmapOrder_SDL_BITMAPORDER_4321,
            BitmapPixelOrder::_1234 => bind::SDL_BitmapOrder_SDL_BITMAPORDER_1234,
        }
    }
}

impl From<bind::SDL_PixelFormatEnum> for BitmapPixelOrder {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        use BitmapPixelOrder::*;
        match (raw >> 20) & 0xf {
            bind::SDL_BitmapOrder_SDL_BITMAPORDER_4321 => _4321,
            bind::SDL_BitmapOrder_SDL_BITMAPORDER_1234 => _1234,
            _ => unreachable!(),
        }
    }
}

/// A pixel order in a packed pixel format.
#[derive(Debug, Clone)]
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
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            PackedPixelOrder::Xrgb => bind::SDL_PackedOrder_SDL_PACKEDORDER_XRGB,
            PackedPixelOrder::Rgbx => bind::SDL_PackedOrder_SDL_PACKEDORDER_RGBX,
            PackedPixelOrder::Argb => bind::SDL_PackedOrder_SDL_PACKEDORDER_ARGB,
            PackedPixelOrder::Rgba => bind::SDL_PackedOrder_SDL_PACKEDORDER_RGBA,
            PackedPixelOrder::Xbgr => bind::SDL_PackedOrder_SDL_PACKEDORDER_XBGR,
            PackedPixelOrder::Bgrx => bind::SDL_PackedOrder_SDL_PACKEDORDER_BGRX,
            PackedPixelOrder::Abgr => bind::SDL_PackedOrder_SDL_PACKEDORDER_ABGR,
            PackedPixelOrder::Bgra => bind::SDL_PackedOrder_SDL_PACKEDORDER_BGRA,
        }
    }
}

impl From<bind::SDL_PixelFormatEnum> for PackedPixelOrder {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        use PackedPixelOrder::*;
        match (raw >> 20) & 0xf {
            bind::SDL_PackedOrder_SDL_PACKEDORDER_XRGB => Xrgb,
            bind::SDL_PackedOrder_SDL_PACKEDORDER_RGBX => Rgbx,
            bind::SDL_PackedOrder_SDL_PACKEDORDER_ARGB => Argb,
            bind::SDL_PackedOrder_SDL_PACKEDORDER_RGBA => Rgba,
            bind::SDL_PackedOrder_SDL_PACKEDORDER_XBGR => Xbgr,
            bind::SDL_PackedOrder_SDL_PACKEDORDER_BGRX => Bgrx,
            bind::SDL_PackedOrder_SDL_PACKEDORDER_ABGR => Abgr,
            bind::SDL_PackedOrder_SDL_PACKEDORDER_BGRA => Bgra,
            _ => unreachable!(),
        }
    }
}

/// A pixel byte order from low byte to high byte for a array pixel format.
#[derive(Debug, Clone)]
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
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            ArrayPixelOrder::Rgb => bind::SDL_ArrayOrder_SDL_ARRAYORDER_RGB,
            ArrayPixelOrder::Rgba => bind::SDL_ArrayOrder_SDL_ARRAYORDER_RGBA,
            ArrayPixelOrder::Argb => bind::SDL_ArrayOrder_SDL_ARRAYORDER_ARGB,
            ArrayPixelOrder::Bgr => bind::SDL_ArrayOrder_SDL_ARRAYORDER_BGR,
            ArrayPixelOrder::Bgra => bind::SDL_ArrayOrder_SDL_ARRAYORDER_BGRA,
            ArrayPixelOrder::Abgr => bind::SDL_ArrayOrder_SDL_ARRAYORDER_ABGR,
        }
    }
}

impl From<bind::SDL_PixelFormatEnum> for ArrayPixelOrder {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        use ArrayPixelOrder::*;
        match (raw >> 20) & 0xf {
            bind::SDL_ArrayOrder_SDL_ARRAYORDER_RGB => Rgb,
            bind::SDL_ArrayOrder_SDL_ARRAYORDER_RGBA => Rgba,
            bind::SDL_ArrayOrder_SDL_ARRAYORDER_ARGB => Argb,
            bind::SDL_ArrayOrder_SDL_ARRAYORDER_BGR => Bgr,
            bind::SDL_ArrayOrder_SDL_ARRAYORDER_BGRA => Bgra,
            bind::SDL_ArrayOrder_SDL_ARRAYORDER_ABGR => Abgr,
            _ => unreachable!(),
        }
    }
}
