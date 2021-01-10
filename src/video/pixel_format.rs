use std::ptr::NonNull;

use kind::PixelFormatKind;

use crate::color::{Rgb, Rgba};
use crate::{bind, Result, Sdl, SdlError};

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
    format: NonNull<bind::SDL_PixelFormat>,
}

impl PixelFormat {
    pub fn new(kind: PixelFormatKind) -> Result<Self> {
        NonNull::new(unsafe { bind::SDL_AllocFormat(kind.into()) }).map_or_else(
            || {
                let msg = Sdl::error();
                Err(if msg == "Out of memory" {
                    SdlError::OutOfMemory
                } else {
                    SdlError::Others { msg }
                })
            },
            |format| Ok(Self { format }),
        )
    }

    pub fn kind(&self) -> PixelFormatKind {
        unsafe { self.format.as_ref() }.format.into()
    }

    pub fn bits_per_pixel(&self) -> u8 {
        unsafe { self.format.as_ref() }.BitsPerPixel
    }

    pub fn bytes_per_pixel(&self) -> u8 {
        unsafe { self.format.as_ref() }.BytesPerPixel
    }

    pub fn components(&self) -> PixelComponents {
        let raw = unsafe { self.format.as_ref() };
        NonNull::new(raw.palette).map_or_else(
            || PixelComponents::TrueColor {
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
            |palette| PixelComponents::PaletteIndex { palette },
        )
    }

    pub fn pixel_by_rgb(&self, Rgb { r, g, b }: Rgb) -> u32 {
        unsafe { bind::SDL_MapRGB(self.format.as_ptr(), r, g, b) }
    }

    pub fn pixel_by_rgba(&self, Rgba { r, g, b, a }: Rgba) -> u32 {
        unsafe { bind::SDL_MapRGBA(self.format.as_ptr(), r, g, b, a) }
    }
}

impl Drop for PixelFormat {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeFormat(self.format.as_ptr()) }
    }
}
