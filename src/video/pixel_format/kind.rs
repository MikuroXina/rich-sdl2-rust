use std::ffi::CStr;

use crate::bind;

#[derive(Debug, Clone)]
pub enum BitmapPixelType {
    Index1,
    Index4,
    Index8,
}

impl BitmapPixelType {
    fn as_raw(&self) -> u32 {
        match self {
            BitmapPixelType::Index1 => bind::SDL_PixelType_SDL_PIXELTYPE_INDEX1,
            BitmapPixelType::Index4 => bind::SDL_PixelType_SDL_PIXELTYPE_INDEX4,
            BitmapPixelType::Index8 => bind::SDL_PixelType_SDL_PIXELTYPE_INDEX8,
        }
    }

    fn bits_per_pixel(&self) -> u32 {
        match self {
            BitmapPixelType::Index1 => 1,
            BitmapPixelType::Index4 => 4,
            BitmapPixelType::Index8 => 8,
        }
    }

    fn bytes_per_pixel(&self) -> u32 {
        if let BitmapPixelType::Index8 = self {
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub enum PackedPixelType {
    _8,
    _16,
    _32,
}

impl PackedPixelType {
    fn as_raw(&self) -> u32 {
        match self {
            PackedPixelType::_8 => bind::SDL_PixelType_SDL_PIXELTYPE_PACKED8,
            PackedPixelType::_16 => bind::SDL_PixelType_SDL_PIXELTYPE_PACKED16,
            PackedPixelType::_32 => bind::SDL_PixelType_SDL_PIXELTYPE_PACKED32,
        }
    }

    fn bytes_per_pixel(&self) -> u32 {
        match self {
            PackedPixelType::_8 => 1,
            PackedPixelType::_16 => 2,
            PackedPixelType::_32 => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArrayPixelType {
    U8,
    U16,
    U32,
    F16,
    F32,
}

impl ArrayPixelType {
    fn as_raw(&self) -> u32 {
        match self {
            ArrayPixelType::U8 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU8,
            ArrayPixelType::U16 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU16,
            ArrayPixelType::U32 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYU32,
            ArrayPixelType::F16 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYF16,
            ArrayPixelType::F32 => bind::SDL_PixelType_SDL_PIXELTYPE_ARRAYF32,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BitmapPixelOrder {
    None,
    _4321,
    _1234,
}

impl BitmapPixelOrder {
    fn as_raw(&self) -> u32 {
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
    fn as_raw(&self) -> u32 {
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
    fn as_raw(&self) -> u32 {
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

#[derive(Debug, Clone)]
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

impl PackedPixelLayout {
    fn as_raw(&self) -> u32 {
        match self {
            PackedPixelLayout::None => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_NONE,
            PackedPixelLayout::_332 => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_332,
            PackedPixelLayout::_4444 => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_4444,
            PackedPixelLayout::_1555 => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_1555,
            PackedPixelLayout::_5551 => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_5551,
            PackedPixelLayout::_565 => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_565,
            PackedPixelLayout::_8888 => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_8888,
            PackedPixelLayout::_2101010 => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_2101010,
            PackedPixelLayout::_1010102 => bind::SDL_PackedLayout_SDL_PACKEDLAYOUT_1010102,
        }
    }
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Default)]
pub struct BppMask {
    pub bpp: std::os::raw::c_int,
    pub r_mask: u32,
    pub g_mask: u32,
    pub b_mask: u32,
    pub a_mask: u32,
}

impl PixelFormatKind {
    pub fn from_bpp_mask(
        BppMask {
            bpp,
            r_mask,
            g_mask,
            b_mask,
            a_mask,
        }: BppMask,
    ) -> Self {
        let raw = unsafe { bind::SDL_MasksToPixelFormatEnum(bpp, r_mask, g_mask, b_mask, a_mask) };
        raw.into()
    }

    pub fn to_bpp_mask(&self) -> Option<BppMask> {
        let mut bpp_mask = BppMask::default();
        let BppMask {
            ref mut bpp,
            ref mut r_mask,
            ref mut g_mask,
            ref mut b_mask,
            ref mut a_mask,
        } = bpp_mask;
        let ret = unsafe {
            bind::SDL_PixelFormatEnumToMasks(
                self.clone().into(),
                bpp as *mut _,
                r_mask as *mut _,
                g_mask as *mut _,
                b_mask as *mut _,
                a_mask as *mut _,
            )
        };
        if ret != 0 {
            Some(bpp_mask)
        } else {
            None
        }
    }

    pub fn name(&self) -> &'static str {
        unsafe { CStr::from_ptr(bind::SDL_GetPixelFormatName(self.clone().into())) }
            .to_str()
            .unwrap_or_default()
    }
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

fn bits_per_packed_pixel(order: PackedPixelOrder, layout: PackedPixelLayout) -> u32 {
    match (order, layout) {
        (PackedPixelOrder::None, _) => 0,
        (_, PackedPixelLayout::None) => 0,
        (PackedPixelOrder::Xrgb, PackedPixelLayout::_332) => 8,
        (PackedPixelOrder::Xrgb, PackedPixelLayout::_4444) => 12,
        (PackedPixelOrder::Xrgb, PackedPixelLayout::_1555) => 15,
        (PackedPixelOrder::Xrgb, PackedPixelLayout::_565) => 16,
        (PackedPixelOrder::Xrgb, PackedPixelLayout::_8888) => 24,
        (PackedPixelOrder::Rgbx, PackedPixelLayout::_8888) => 24,
        (PackedPixelOrder::Argb, PackedPixelLayout::_4444) => 16,
        (PackedPixelOrder::Argb, PackedPixelLayout::_1555) => 16,
        (PackedPixelOrder::Argb, PackedPixelLayout::_5551) => 16,
        (PackedPixelOrder::Argb, PackedPixelLayout::_8888) => 32,
        (PackedPixelOrder::Argb, PackedPixelLayout::_2101010) => 32,
        (PackedPixelOrder::Rgba, PackedPixelLayout::_4444) => 32,
        (PackedPixelOrder::Rgba, PackedPixelLayout::_5551) => 16,
        (PackedPixelOrder::Rgba, PackedPixelLayout::_8888) => 32,
        (PackedPixelOrder::Xbgr, PackedPixelLayout::_4444) => 12,
        (PackedPixelOrder::Xbgr, PackedPixelLayout::_1555) => 15,
        (PackedPixelOrder::Xbgr, PackedPixelLayout::_565) => 16,
        (PackedPixelOrder::Xbgr, PackedPixelLayout::_8888) => 24,
        (PackedPixelOrder::Bgrx, PackedPixelLayout::_8888) => 24,
        (PackedPixelOrder::Abgr, PackedPixelLayout::_4444) => 16,
        (PackedPixelOrder::Abgr, PackedPixelLayout::_1555) => 16,
        (PackedPixelOrder::Abgr, PackedPixelLayout::_8888) => 32,
        (PackedPixelOrder::Bgra, PackedPixelLayout::_4444) => 16,
        (PackedPixelOrder::Bgra, PackedPixelLayout::_5551) => 16,
        (PackedPixelOrder::Bgra, PackedPixelLayout::_8888) => 32,
        _ => 0,
    }
}

fn bytes_per_array_pixel(ty: &ArrayPixelType, order: &ArrayPixelOrder) -> u32 {
    let components = match order {
        ArrayPixelOrder::None => return 0,
        ArrayPixelOrder::Rgb | ArrayPixelOrder::Bgr => 3,
        ArrayPixelOrder::Rgba
        | ArrayPixelOrder::Argb
        | ArrayPixelOrder::Bgra
        | ArrayPixelOrder::Abgr => 4,
    };
    let bits_per_component = match ty {
        ArrayPixelType::U8 => 8,
        ArrayPixelType::U16 => 16,
        ArrayPixelType::U32 => 32,
        ArrayPixelType::F16 => 16,
        ArrayPixelType::F32 => 32,
    };
    components * bits_per_component
}

fn calc_bits(ty: u32, order: u32, layout: u32, bits_per_pixel: u32, bytes_per_pixel: u32) -> u32 {
    1 << 28 | ty << 24 | order << 20 | layout << 16 | bits_per_pixel << 8 | bytes_per_pixel << 0
}

impl From<PixelFormatKind> for bind::SDL_PixelFormatEnum {
    fn from(kind: PixelFormatKind) -> Self {
        match kind {
            PixelFormatKind::Unknown => 0,
            PixelFormatKind::Bitmap { ty, order } => calc_bits(
                ty.as_raw(),
                order.as_raw(),
                0,
                ty.bits_per_pixel(),
                ty.bytes_per_pixel(),
            ),
            PixelFormatKind::Packed { ty, order, layout } => calc_bits(
                ty.as_raw(),
                order.as_raw(),
                layout.as_raw(),
                bits_per_packed_pixel(order, layout),
                ty.bytes_per_pixel(),
            ),
            PixelFormatKind::Array { ty, order } => {
                let bits = bytes_per_array_pixel(&ty, &order);
                calc_bits(ty.as_raw(), order.as_raw(), 0, bits, bits / 8)
            }
            PixelFormatKind::FourCode(code) => {
                let bytes: Vec<_> = code.bytes().map(|byte| byte as u32).collect();
                bytes[0] | bytes[1] << 8 | bytes[2] << 16 | bytes[3] << 24
            }
        }
    }
}
