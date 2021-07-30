use crate::bind;

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
    pub(super) fn as_raw(&self) -> u32 {
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
