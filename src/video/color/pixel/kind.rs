//! Kinds for a pixel format, aggregates type, layout and order.

use std::ffi::CStr;

use crate::{bind, EnumInt};

use super::{
    layout::PackedPixelLayout,
    order::{ArrayPixelOrder, BitmapPixelOrder, PackedPixelOrder},
    ty::{ArrayPixelType, BitmapPixelType, PackedPixelType},
};

/// A kind of pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PixelFormatKind {
    /// A format not supported by SDL2.
    Unknown,
    /// An indexed bitmap format with a palette.
    Bitmap {
        /// The type of the bitmap.
        ty: BitmapPixelType,
        /// The type of the bitmap.
        order: BitmapPixelOrder,
    },
    /// A packed pixel format.
    Packed {
        /// The type of the pack.
        ty: PackedPixelType,
        /// The order of the pack.
        order: PackedPixelOrder,
        /// The layout of the pack.
        layout: PackedPixelLayout,
    },
    /// A pixel array format.
    Array {
        /// The type of the array.
        ty: ArrayPixelType,
        /// The order of the array.
        order: ArrayPixelOrder,
    },
    /// A special format such as YUV in FourCC code.
    FourCode([u8; 4]),
}

/// Bpp and RGBA mask.
#[derive(Debug, Default)]
pub struct BppMask {
    /// Bits per pixel, normally 15, 16 or 32.
    pub bpp: std::os::raw::c_int,
    /// Mask for the red component.
    pub r_mask: u32,
    /// Mask for the green component.
    pub g_mask: u32,
    /// Mask for the blue component.
    pub b_mask: u32,
    /// Mask for the alpha component.
    pub a_mask: u32,
}

impl PixelFormatKind {
    /// Constructs from [`BppMask`].
    #[must_use]
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
        Self::from_raw(raw as EnumInt)
    }

    /// Converts to [`BppMask`], if able to do.
    #[must_use]
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
                self.as_raw(),
                bpp as *mut _,
                r_mask as *mut _,
                g_mask as *mut _,
                b_mask as *mut _,
                a_mask as *mut _,
            )
        };
        (ret != 0).then(|| bpp_mask)
    }

    /// Returns the name for the pixel format, or empty string if does not exist.
    #[must_use]
    pub fn name(&self) -> &'static str {
        unsafe { CStr::from_ptr(bind::SDL_GetPixelFormatName(self.as_raw())) }
            .to_str()
            .unwrap_or_default()
    }

    pub(crate) fn from_raw(raw: EnumInt) -> Self {
        if (raw >> 28) & 0x0F != 1 {
            let bytes = ((raw >> 24) & 0xf).to_le_bytes();
            return PixelFormatKind::FourCode(bytes);
        }
        match (raw >> 24) & 0xf {
            bind::SDL_PIXELTYPE_INDEX1 => PixelFormatKind::Bitmap {
                ty: BitmapPixelType::Index1,
                order: raw.into(),
            },
            bind::SDL_PIXELTYPE_INDEX4 => PixelFormatKind::Bitmap {
                ty: BitmapPixelType::Index4,
                order: raw.into(),
            },
            bind::SDL_PIXELTYPE_INDEX8 => PixelFormatKind::Bitmap {
                ty: BitmapPixelType::Index8,
                order: raw.into(),
            },
            bind::SDL_PIXELTYPE_PACKED8 => PixelFormatKind::Packed {
                ty: PackedPixelType::_8,
                order: raw.into(),
                layout: raw.into(),
            },
            bind::SDL_PIXELTYPE_PACKED16 => PixelFormatKind::Packed {
                ty: PackedPixelType::_16,
                order: raw.into(),
                layout: raw.into(),
            },
            bind::SDL_PIXELTYPE_PACKED32 => PixelFormatKind::Packed {
                ty: PackedPixelType::_32,
                order: raw.into(),
                layout: raw.into(),
            },
            bind::SDL_PIXELTYPE_ARRAYU8 => PixelFormatKind::Array {
                ty: ArrayPixelType::U8,
                order: raw.into(),
            },
            bind::SDL_PIXELTYPE_ARRAYU16 => PixelFormatKind::Array {
                ty: ArrayPixelType::U16,
                order: raw.into(),
            },
            bind::SDL_PIXELTYPE_ARRAYU32 => PixelFormatKind::Array {
                ty: ArrayPixelType::U32,
                order: raw.into(),
            },
            bind::SDL_PIXELTYPE_ARRAYF16 => PixelFormatKind::Array {
                ty: ArrayPixelType::F16,
                order: raw.into(),
            },
            bind::SDL_PIXELTYPE_ARRAYF32 => PixelFormatKind::Array {
                ty: ArrayPixelType::F32,
                order: raw.into(),
            },
            _ => PixelFormatKind::Unknown,
        }
    }

    pub(crate) fn as_raw(self) -> u32 {
        match self {
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
                let bits = bytes_per_array_pixel(ty, order);
                calc_bits(ty.as_raw(), order.as_raw(), 0, bits, bits / 8)
            }
            PixelFormatKind::FourCode(bytes) => u32::from_le_bytes(bytes),
        }
    }
}

fn bits_per_packed_pixel(order: PackedPixelOrder, layout: PackedPixelLayout) -> u32 {
    match (order, layout) {
        (PackedPixelOrder::Xrgb, PackedPixelLayout::_332) => 8,
        (PackedPixelOrder::Xbgr | PackedPixelOrder::Xrgb, PackedPixelLayout::_4444) => 12,
        (PackedPixelOrder::Xrgb | PackedPixelOrder::Xbgr, PackedPixelLayout::_1555) => 15,
        (PackedPixelOrder::Abgr | PackedPixelOrder::Argb, PackedPixelLayout::_1555)
        | (
            PackedPixelOrder::Abgr | PackedPixelOrder::Argb | PackedPixelOrder::Bgra,
            PackedPixelLayout::_4444,
        )
        | (
            PackedPixelOrder::Argb | PackedPixelOrder::Bgra | PackedPixelOrder::Rgba,
            PackedPixelLayout::_5551,
        )
        | (PackedPixelOrder::Xbgr | PackedPixelOrder::Xrgb, PackedPixelLayout::_565) => 16,
        (
            PackedPixelOrder::Rgbx
            | PackedPixelOrder::Bgrx
            | PackedPixelOrder::Xbgr
            | PackedPixelOrder::Xrgb,
            PackedPixelLayout::_8888,
        ) => 24,
        (PackedPixelOrder::Rgba, PackedPixelLayout::_4444)
        | (
            PackedPixelOrder::Abgr
            | PackedPixelOrder::Argb
            | PackedPixelOrder::Bgra
            | PackedPixelOrder::Rgba,
            PackedPixelLayout::_8888,
        )
        | (PackedPixelOrder::Argb, PackedPixelLayout::_2101010) => 32,
        _ => 0,
    }
}

fn bytes_per_array_pixel(ty: ArrayPixelType, order: ArrayPixelOrder) -> u32 {
    let components = match order {
        ArrayPixelOrder::Rgb | ArrayPixelOrder::Bgr => 3,
        ArrayPixelOrder::Rgba
        | ArrayPixelOrder::Argb
        | ArrayPixelOrder::Bgra
        | ArrayPixelOrder::Abgr => 4,
    };
    let bits_per_component = match ty {
        ArrayPixelType::U8 => 8,
        ArrayPixelType::U16 | ArrayPixelType::F16 => 16,
        ArrayPixelType::U32 | ArrayPixelType::F32 => 32,
    };
    components * bits_per_component
}

fn calc_bits(ty: u32, order: u32, layout: u32, bits_per_pixel: u32, bytes_per_pixel: u32) -> u32 {
    1 << 28 | ty << 24 | order << 20 | layout << 16 | bits_per_pixel << 8 | bytes_per_pixel
}
