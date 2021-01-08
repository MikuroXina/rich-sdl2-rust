use crate::bind;

#[derive(Debug)]
pub enum BitmapPixelType {
    Index1,
    Index4,
    Index8,
}

#[derive(Debug)]
pub enum PackedPixelType {
    _8,
    _16,
    _32,
}

#[derive(Debug)]
pub enum ArrayPixelType {
    U8,
    U16,
    U32,
    F16,
    F32,
}

#[derive(Debug)]
pub enum BitmapPixelOrder {
    None,
    _4321,
    _1234,
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ArrayPixelOrder {
    None,
    Rgb,
    Rgba,
    Argb,
    Bgr,
    Bgra,
    Abgr,
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

#[derive(Debug)]
pub enum PackedPixelLayout {
    None,
    _332,
    _4444,
    _1555,
    _5551,
    _565,
    _8888,
    _2101010,
    _1010102,
}

impl From<bind::SDL_PixelFormatEnum> for PackedPixelLayout {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        use PackedPixelLayout::*;
        match (raw >> 16) & 0xf {
            bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_332 => _332,
            bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_4444 => _4444,
            bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_1555 => _1555,
            bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_5551 => _5551,
            bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_565 => _565,
            bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_8888 => _8888,
            bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_2101010 => _2101010,
            bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_1010102 => _1010102,
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PixelFormatKind {
    Unknown,
    Bitmap {
        ty: BitmapPixelType,
        order: BitmapPixelOrder,
    },
    Packed {
        ty: PackedPixelType,
        order: PackedPixelOrder,
        layout: PackedPixelLayout,
    },
    Array {
        ty: ArrayPixelType,
        order: ArrayPixelOrder,
    },
    FourCode(String),
}

impl From<bind::SDL_PixelFormatEnum> for PixelFormatKind {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        use PixelFormatKind::*;
        if (raw >> 28) & 0x0F != 1 {
            let bytes = ((raw >> 24) & 0xf).to_le_bytes();
            return FourCode(bytes.iter().map(|&c| c as char).collect());
        }
        match (raw >> 24) & 0xf {
            bind::SDL_PixelType_SDL_PIXELTYPE_INDEX1 => Bitmap {
                ty: BitmapPixelType::Index1,
                order: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_INDEX4 => Bitmap {
                ty: BitmapPixelType::Index4,
                order: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_INDEX8 => Bitmap {
                ty: BitmapPixelType::Index8,
                order: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_PACKED8 => Packed {
                ty: PackedPixelType::_8,
                order: raw.into(),
                layout: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_PACKED16 => Packed {
                ty: PackedPixelType::_16,
                order: raw.into(),
                layout: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_PACKED32 => Packed {
                ty: PackedPixelType::_32,
                order: raw.into(),
                layout: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU8 => Array {
                ty: ArrayPixelType::U8,
                order: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU16 => Array {
                ty: ArrayPixelType::U16,
                order: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU32 => Array {
                ty: ArrayPixelType::U32,
                order: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYF16 => Array {
                ty: ArrayPixelType::F16,
                order: raw.into(),
            },
            bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYF32 => Array {
                ty: ArrayPixelType::F32,
                order: raw.into(),
            },
            _ => Unknown,
        }
    }
}

impl From<PixelFormatKind> for bind::SDL_PixelFormat {
    fn from(fmt: PixelFormatKind) -> Self {
        todo!()
    }
}
