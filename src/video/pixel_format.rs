use std::ptr::NonNull;

use kind::PixelFormatKind;

use crate::bind;

pub mod kind;

pub struct PixelComponent {
    pub mask: u32,
    pub loss: u8,
    pub shift: u8,
}

pub enum PixelComponents {
    PaletteIndex {
        palette: NonNull<bind::SDL_Palette>,
    },
    TrueColor {
        red: PixelComponent,
        green: PixelComponent,
        blue: PixelComponent,
        alpha: PixelComponent,
    },
}

pub struct PixelFormat {
    pub kind: PixelFormatKind,
    pub bits_per_pixel: u8,
    pub bytes_per_pixel: u8,
    pub components: PixelComponents,
}

impl PixelFormat {
    pub(crate) fn from_raw(flags: bind::SDL_PixelFormatEnum, raw: bind::SDL_PixelFormat) -> Self {
        NonNull::new(raw.palette).map_or_else(
            || Self {
                kind: flags.into(),
                bits_per_pixel: raw.BitsPerPixel,
                bytes_per_pixel: raw.BytesPerPixel,
                components: PixelComponents::TrueColor {
                    red: PixelComponent {
                        mask: raw.Rmask,
                        loss: raw.Rloss,
                        shift: raw.Rshift,
                    },
                    green: PixelComponent {
                        mask: raw.Gmask,
                        loss: raw.Gloss,
                        shift: raw.Gshift,
                    },
                    blue: PixelComponent {
                        mask: raw.Bmask,
                        loss: raw.Bloss,
                        shift: raw.Bshift,
                    },
                    alpha: PixelComponent {
                        mask: raw.Amask,
                        loss: raw.Aloss,
                        shift: raw.Ashift,
                    },
                },
            },
            |palette| Self {
                kind: flags.into(),
                bits_per_pixel: raw.BitsPerPixel,
                bytes_per_pixel: raw.BytesPerPixel,
                components: PixelComponents::PaletteIndex { palette },
            },
        )
    }
}
