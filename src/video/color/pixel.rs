//! Pixel definitions for operating colors.
use std::ptr::NonNull;

use kind::PixelFormatKind;
use palette::Palette;
use static_assertions::assert_not_impl_all;

use crate::color::{Rgb, Rgba};
use crate::{bind, EnumInt, Result, Sdl, SdlError};

pub mod kind;
pub mod layout;
pub mod order;
pub mod palette;
pub mod ty;

/// A simple pixel that can convert into `u32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[must_use]
pub struct Pixel {
    pixel: u32,
}

impl Pixel {
    /// Convert into `u32`.
    #[must_use]
    pub fn as_u32(self) -> u32 {
        self.pixel
    }
}

/// A bit mask to extract a component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixelMask(pub u32);

/// A property of a pixel format.
#[derive(Debug)]
#[non_exhaustive]
pub enum PixelFormatProperty {
    /// A pixel format with a palette, so a pixel has an index of the palette.
    Palette(Palette),
    /// A pixel has own color information.
    TrueColor {
        /// A mask of red component.
        red: PixelMask,
        /// A mask of green component.
        green: PixelMask,
        /// A mask of blue component.
        blue: PixelMask,
        /// A mask of alpha component.
        alpha: PixelMask,
    },
}

/// A pixel format.
pub struct PixelFormat {
    format: NonNull<bind::SDL_PixelFormat>,
}

impl std::fmt::Debug for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelFormat")
            .field("kind", &self.kind())
            .finish()
    }
}

assert_not_impl_all!(PixelFormat: Send, Sync);

impl PixelFormat {
    /// Constructs a pixel format from [`PixelFormatKind`].
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to allocate the memory for the format, or construct the format.
    pub fn new(kind: PixelFormatKind) -> Result<Self> {
        NonNull::new(unsafe { bind::SDL_AllocFormat(kind.as_raw()) }).map_or_else(
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

    /// Returns the kind of the format.
    #[must_use]
    pub fn kind(&self) -> PixelFormatKind {
        PixelFormatKind::from_raw(unsafe { self.format.as_ref() }.format as EnumInt)
    }

    /// Returns the bits per pixel of the format.
    #[must_use]
    pub fn bits_per_pixel(&self) -> u8 {
        unsafe { self.format.as_ref() }.BitsPerPixel
    }

    /// Returns the bytes per pixel of the format.
    #[must_use]
    pub fn bytes_per_pixel(&self) -> u8 {
        unsafe { self.format.as_ref() }.BytesPerPixel
    }

    /// Returns the property of the format.
    #[must_use]
    pub fn property(&self) -> PixelFormatProperty {
        let raw = unsafe { self.format.as_ref() };
        NonNull::new(raw.palette).map_or_else(
            || PixelFormatProperty::TrueColor {
                red: PixelMask(raw.Rmask),
                green: PixelMask(raw.Gmask),
                blue: PixelMask(raw.Bmask),
                alpha: PixelMask(raw.Amask),
            },
            |palette| PixelFormatProperty::Palette(Palette { palette }),
        )
    }

    /// Converts [`Rgb`] into [`Pixel`].
    pub fn pixel_by_rgb(&self, Rgb { r, g, b }: Rgb) -> Pixel {
        let pixel = unsafe { bind::SDL_MapRGB(self.format.as_ptr(), r, g, b) };
        Pixel { pixel }
    }

    /// Converts [`Rgba`] into [`Pixel`].
    pub fn pixel_by_rgba(&self, Rgba { r, g, b, a }: Rgba) -> Pixel {
        let pixel = unsafe { bind::SDL_MapRGBA(self.format.as_ptr(), r, g, b, a) };
        Pixel { pixel }
    }

    /// Converts [`Pixel`] into [`Rgb`].
    pub fn rgb_from_pixel(&self, Pixel { pixel }: Pixel) -> Rgb {
        let mut rgb = Rgb { r: 0, g: 0, b: 0 };
        unsafe {
            bind::SDL_GetRGB(
                pixel,
                self.format.as_ptr(),
                &mut rgb.r,
                &mut rgb.g,
                &mut rgb.b,
            );
        }
        rgb
    }

    /// Converts [`Pixel`] into [`Rgba`].
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
                &mut rgba.r,
                &mut rgba.g,
                &mut rgba.b,
                &mut rgba.a,
            );
        }
        rgba
    }

    /// Overwrites the palette with a new [`Palette`].
    pub fn set_palette(&self, palette: &Palette) {
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
