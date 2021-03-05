use std::ptr::NonNull;

use kind::PixelFormatKind;
use palette::Palette;

use crate::color::{Rgb, Rgba};
use crate::{bind, Result, Sdl, SdlError};

pub mod kind;
pub mod palette;

#[derive(Debug, Clone)]
pub struct Pixel {
    pixel: u32,
}

impl Pixel {
    pub fn as_u32(&self) -> u32 {
        self.pixel
    }
}

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

    pub fn pixel_by_rgb(&self, Rgb { r, g, b }: Rgb) -> Pixel {
        let pixel = unsafe { bind::SDL_MapRGB(self.format.as_ptr(), r, g, b) };
        Pixel { pixel }
    }

    pub fn pixel_by_rgba(&self, Rgba { r, g, b, a }: Rgba) -> Pixel {
        let pixel = unsafe { bind::SDL_MapRGBA(self.format.as_ptr(), r, g, b, a) };
        Pixel { pixel }
    }

    pub fn rgb_from_pixel(&self, Pixel { pixel }: Pixel) -> Rgb {
        let mut rgb = Rgb { r: 0, g: 0, b: 0 };
        unsafe {
            bind::SDL_GetRGB(
                pixel,
                self.format.as_ptr(),
                &mut rgb.r as *mut _,
                &mut rgb.g as *mut _,
                &mut rgb.b as *mut _,
            )
        }
        rgb
    }

    pub fn rgba_from_pixel(&self, Pixel { pixel }: Pixel) -> Rgba {
        let mut rgba = Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
        unsafe {
            bind::SDL_GetRGBA(
                pixel,
                self.format.as_ptr(),
                &mut rgba.r as *mut _,
                &mut rgba.g as *mut _,
                &mut rgba.b as *mut _,
                &mut rgba.a as *mut _,
            )
        }
        rgba
    }

    pub fn set_palette(&self, palette: Palette) {
        let ret =
            unsafe { bind::SDL_SetPixelFormatPalette(self.format.as_ptr(), palette.as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Setting pixel format palette");
        }
    }
}

impl Drop for PixelFormat {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeFormat(self.format.as_ptr()) }
    }
}
