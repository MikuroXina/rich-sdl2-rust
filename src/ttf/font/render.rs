use std::{ffi::CString, ptr::NonNull};

use crate::{
    color::Rgba,
    surface::{RawSurface, Surface},
    Result, Sdl, SdlError,
};

use super::Font;
use crate::bind;

pub mod pen;

/// A render mode for the font glyphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RenderMode {
    /// A quick and dirty mode.
    Solid {
        /// A text color.
        foreground: Rgba,
    },
    /// A slow and nice, but with a solid box mode.
    Shaded {
        /// A text color.
        foreground: Rgba,
        /// A background color, around of a text.
        background: Rgba,
    },
    /// A too slow, but ultra nice over others.
    Blended {
        /// A text color.
        foreground: Rgba,
    },
}

/// An extension for rendering the text.
pub trait RenderExt {
    /// Renders the text into the [`TtfSurface`], or `Err` on failure.
    fn render(&self, text: &str, mode: RenderMode) -> Result<TtfSurface>;
    /// Renders the wrapped text into the [`TtfSurface`], or `Err` on failure.
    fn render_wrapped(&self, text: &str, wrap_length: u32, mode: RenderMode) -> Result<TtfSurface>;
    /// Renders the character into the [`TtfSurface`], or `Err` on failure.
    fn render_glyph(&self, ch: char, mode: RenderMode) -> Result<TtfSurface>;
}

impl RenderExt for Font<'_> {
    fn render(&self, text: &str, mode: RenderMode) -> Result<TtfSurface> {
        let cstr = CString::new(text).unwrap_or_default();
        let raw_color = |color: Rgba| bind::SDL_Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
        let ptr = match mode {
            RenderMode::Solid { foreground } => unsafe {
                bind::TTF_RenderUTF8_Solid(self.ptr.as_ptr(), cstr.as_ptr(), raw_color(foreground))
            },
            RenderMode::Shaded {
                foreground,
                background,
            } => unsafe {
                bind::TTF_RenderUTF8_Shaded(
                    self.ptr.as_ptr(),
                    cstr.as_ptr(),
                    raw_color(foreground),
                    raw_color(background),
                )
            },
            RenderMode::Blended { foreground } => unsafe {
                bind::TTF_RenderUTF8_Blended(
                    self.ptr.as_ptr(),
                    cstr.as_ptr(),
                    raw_color(foreground),
                )
            },
        };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(TtfSurface {
                ptr: NonNull::new(ptr.cast()).unwrap(),
            })
        }
    }

    fn render_wrapped(&self, text: &str, wrap_length: u32, mode: RenderMode) -> Result<TtfSurface> {
        let cstr = CString::new(text).unwrap_or_default();
        let raw_color = |color: Rgba| bind::SDL_Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
        let ptr = match mode {
            RenderMode::Solid { foreground } => unsafe {
                bind::TTF_RenderUTF8_Solid_Wrapped(
                    self.ptr.as_ptr(),
                    cstr.as_ptr(),
                    raw_color(foreground),
                    wrap_length,
                )
            },
            RenderMode::Shaded {
                foreground,
                background,
            } => unsafe {
                bind::TTF_RenderUTF8_Shaded_Wrapped(
                    self.ptr.as_ptr(),
                    cstr.as_ptr(),
                    raw_color(foreground),
                    raw_color(background),
                    wrap_length,
                )
            },
            RenderMode::Blended { foreground } => unsafe {
                bind::TTF_RenderUTF8_Blended_Wrapped(
                    self.ptr.as_ptr(),
                    cstr.as_ptr(),
                    raw_color(foreground),
                    wrap_length,
                )
            },
        };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(TtfSurface {
                ptr: NonNull::new(ptr.cast()).unwrap(),
            })
        }
    }

    fn render_glyph(&self, ch: char, mode: RenderMode) -> Result<TtfSurface> {
        let raw_color = |color: Rgba| bind::SDL_Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
        let ptr = match mode {
            RenderMode::Solid { foreground } => unsafe {
                bind::TTF_RenderGlyph32_Solid(self.ptr.as_ptr(), ch as u32, raw_color(foreground))
            },
            RenderMode::Shaded {
                foreground,
                background,
            } => unsafe {
                bind::TTF_RenderGlyph32_Shaded(
                    self.ptr.as_ptr(),
                    ch as u32,
                    raw_color(foreground),
                    raw_color(background),
                )
            },
            RenderMode::Blended { foreground } => unsafe {
                bind::TTF_RenderGlyph32_Blended(self.ptr.as_ptr(), ch as u32, raw_color(foreground))
            },
        };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(TtfSurface {
                ptr: NonNull::new(ptr.cast()).unwrap(),
            })
        }
    }
}

/// A surface of TrueType Font.
pub struct TtfSurface {
    ptr: NonNull<RawSurface>,
}

impl Surface for TtfSurface {
    fn as_ptr(&self) -> NonNull<RawSurface> {
        self.ptr
    }
}

impl Drop for TtfSurface {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeSurface(self.ptr.as_ptr().cast()) }
    }
}
