use crate::bind;

#[derive(Debug, Clone)]
pub enum BitmapPixelOrder {
    None,
    _4321,
    _1234,
}

impl BitmapPixelOrder {
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            BitmapPixelOrder::None => bind::SDL_BitmapOrder_SDL_BITMAPORDER_NONE,
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
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PackedPixelOrder {
    None,
    Xrgb,
    Rgbx,
    Argb,
    Rgba,
    Xbgr,
    Bgrx,
    Abgr,
    Bgra,
}

impl PackedPixelOrder {
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            PackedPixelOrder::None => bind::SDL_PackedOrder_SDL_PACKEDORDER_NONE,
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
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArrayPixelOrder {
    None,
    Rgb,
    Rgba,
    Argb,
    Bgr,
    Bgra,
    Abgr,
}

impl ArrayPixelOrder {
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            ArrayPixelOrder::None => bind::SDL_ArrayOrder_SDL_ARRAYORDER_NONE,
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
            _ => None,
        }
    }
}
