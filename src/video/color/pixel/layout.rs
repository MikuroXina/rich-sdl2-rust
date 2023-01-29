//! Layouts for a pixel format.

use crate::bind;

/// A layout for a packed pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PackedPixelLayout {
    /// A layout is in 8 bits with 4 components, as below:
    ///
    /// ```text
    /// 1112 2233
    /// ```
    _332,
    /// A layout is in 16 bits with 4 components with same size, as below:
    ///
    /// ```text
    /// 0000 1111 2222 3333
    /// ```
    _4444,
    /// A layout is in 16 bits with 4 components where only one component is 1 bit but others is 5 bits, as below:
    ///
    /// ```text
    /// 01111 1222 2233 3333
    /// ```
    _1555,
    /// A layout is in 16 bits with 4 components where only one component is 1 bit but others is 5 bits, as below:
    ///
    /// ```text
    /// 0000 0111 1122 2223
    /// ```
    _5551,
    /// A layout is in 16 bits with 3 components where only one component is 6 bits but others is 5 bits, as below:
    ///
    /// ```text
    /// 0000 0111 1112 2222
    /// ```
    _565,
    /// A layout is in 32 bits with 4 components with same size, as below:
    ///
    /// ```text
    /// 0000 0000 1111 1111 2222 2222 3333 3333
    /// ```
    _8888,
    /// A layout is in 32 bits with 4 components where only one component is 2 bits but others is 10 bits, as below:
    ///
    /// ```text
    /// 0011 1111 1111 2222 2222 2233 3333 3333
    /// ```
    _2101010,
    /// A layout is in 32 bits with 4 components where only one component is 2 bits but others is 10 bits, as below:
    ///
    /// ```text
    /// 0000 0000 0011 1111 1111 2222 2222 2233
    /// ```
    _1010102,
}

impl PackedPixelLayout {
    #[allow(clippy::unnecessary_cast)]
    pub(super) fn as_raw(self) -> u32 {
        (match self {
            PackedPixelLayout::_332 => bind::SDL_PACKEDLAYOUT_332,
            PackedPixelLayout::_4444 => bind::SDL_PACKEDLAYOUT_4444,
            PackedPixelLayout::_1555 => bind::SDL_PACKEDLAYOUT_1555,
            PackedPixelLayout::_5551 => bind::SDL_PACKEDLAYOUT_5551,
            PackedPixelLayout::_565 => bind::SDL_PACKEDLAYOUT_565,
            PackedPixelLayout::_8888 => bind::SDL_PACKEDLAYOUT_8888,
            PackedPixelLayout::_2101010 => bind::SDL_PACKEDLAYOUT_2101010,
            PackedPixelLayout::_1010102 => bind::SDL_PACKEDLAYOUT_1010102,
        }) as u32
    }
}

impl From<bind::SDL_PixelFormatEnum> for PackedPixelLayout {
    fn from(raw: bind::SDL_PixelFormatEnum) -> Self {
        match (raw >> 16) & 0xf {
            bind::SDL_PACKEDLAYOUT_332 => PackedPixelLayout::_332,
            bind::SDL_PACKEDLAYOUT_4444 => PackedPixelLayout::_4444,
            bind::SDL_PACKEDLAYOUT_1555 => PackedPixelLayout::_1555,
            bind::SDL_PACKEDLAYOUT_5551 => PackedPixelLayout::_5551,
            bind::SDL_PACKEDLAYOUT_565 => PackedPixelLayout::_565,
            bind::SDL_PACKEDLAYOUT_8888 => PackedPixelLayout::_8888,
            bind::SDL_PACKEDLAYOUT_2101010 => PackedPixelLayout::_2101010,
            bind::SDL_PACKEDLAYOUT_1010102 => PackedPixelLayout::_1010102,
            _ => unreachable!(),
        }
    }
}
