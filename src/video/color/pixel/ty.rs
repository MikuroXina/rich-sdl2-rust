use crate::bind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapPixelType {
    Index1,
    Index4,
    Index8,
}

impl BitmapPixelType {
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            BitmapPixelType::Index1 => bind::SDL_PixelType_SDL_PIXELTYPE_INDEX1,
            BitmapPixelType::Index4 => bind::SDL_PixelType_SDL_PIXELTYPE_INDEX4,
            BitmapPixelType::Index8 => bind::SDL_PixelType_SDL_PIXELTYPE_INDEX8,
        }
    }

    pub(super) fn bits_per_pixel(&self) -> u32 {
        match self {
            BitmapPixelType::Index1 => 1,
            BitmapPixelType::Index4 => 4,
            BitmapPixelType::Index8 => 8,
        }
    }

    pub(super) fn bytes_per_pixel(&self) -> u32 {
        if let BitmapPixelType::Index8 = self {
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackedPixelType {
    _8,
    _16,
    _32,
}

impl PackedPixelType {
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            PackedPixelType::_8 => bind::SDL_PixelType_SDL_PIXELTYPE_PACKED8,
            PackedPixelType::_16 => bind::SDL_PixelType_SDL_PIXELTYPE_PACKED16,
            PackedPixelType::_32 => bind::SDL_PixelType_SDL_PIXELTYPE_PACKED32,
        }
    }

    pub(super) fn bytes_per_pixel(&self) -> u32 {
        match self {
            PackedPixelType::_8 => 1,
            PackedPixelType::_16 => 2,
            PackedPixelType::_32 => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayPixelType {
    U8,
    U16,
    U32,
    F16,
    F32,
}

impl ArrayPixelType {
    pub(super) fn as_raw(&self) -> u32 {
        match self {
            ArrayPixelType::U8 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU8,
            ArrayPixelType::U16 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU16,
            ArrayPixelType::U32 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU32,
            ArrayPixelType::F16 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYF16,
            ArrayPixelType::F32 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYF32,
        }
    }
}
