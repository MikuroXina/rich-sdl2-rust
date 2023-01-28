use std::os::raw::c_int;

use super::Font;
use crate::{bind, Result, SdlError};

/// A hinting for a font.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FontHinting {
    /// Any hinting is not set.
    Normal,
    /// A light hinting.
    Light,
    /// A monochrome hinting.
    Mono,
    /// A disabled hinting.
    None,
    /// A light and subpixel hinting.
    LightSubpixel,
}

impl FontHinting {
    fn from_raw(raw: c_int) -> Self {
        match raw as u32 {
            bind::TTF_HINTING_NORMAL => Self::Normal,
            bind::TTF_HINTING_LIGHT => Self::Light,
            bind::TTF_HINTING_MONO => Self::Mono,
            bind::TTF_HINTING_NONE => Self::None,
            bind::TTF_HINTING_LIGHT_SUBPIXEL => Self::LightSubpixel,
            _ => unreachable!(),
        }
    }

    fn into_raw(self) -> c_int {
        (match self {
            Self::Normal => bind::TTF_HINTING_NORMAL,
            Self::Light => bind::TTF_HINTING_LIGHT,
            Self::Mono => bind::TTF_HINTING_MONO,
            Self::None => bind::TTF_HINTING_NONE,
            Self::LightSubpixel => bind::TTF_HINTING_LIGHT_SUBPIXEL,
        }) as _
    }
}

/// A disabler of font kerning. Kerning of the font is disabled until this struct is dropped.
pub struct KerningDisabler<'font>(&'font Font<'font>);

impl Drop for KerningDisabler<'_> {
    fn drop(&mut self) {
        unsafe { bind::TTF_SetFontKerning(self.0.ptr.as_ptr(), 1) }
    }
}

/// An extension for settings of the font.
pub trait FontSetting {
    /// Returns the current hinting of the font.
    fn hinting(&self) -> FontHinting;
    /// Sets the hinting of the font.
    fn set_hinting(&self, hinting: FontHinting);

    /// Disables the font kerning.
    fn disable_kerning(&self) -> KerningDisabler;

    /// Returns whether the font supports Singed Distance Field.
    fn is_sdf(&self) -> bool;
    /// Sets whether the font supports Singed Distance Field.
    fn set_sdf(&self, value: bool) -> Result<()>;
}

impl FontSetting for Font<'_> {
    fn hinting(&self) -> FontHinting {
        let raw = unsafe { bind::TTF_GetFontHinting(self.ptr.as_ptr()) };
        FontHinting::from_raw(raw)
    }

    fn set_hinting(&self, hinting: FontHinting) {
        // needed to check to prevent cache from erasing.
        if hinting != self.hinting() {
            unsafe { bind::TTF_SetFontHinting(self.ptr.as_ptr(), hinting.into_raw()) }
        }
    }

    fn disable_kerning(&self) -> KerningDisabler {
        unsafe { bind::TTF_SetFontKerning(self.ptr.as_ptr(), 1) }
        KerningDisabler(self)
    }

    fn is_sdf(&self) -> bool {
        unsafe { bind::TTF_GetFontSDF(self.ptr.as_ptr()) == bind::SDL_TRUE }
    }

    fn set_sdf(&self, value: bool) -> Result<()> {
        // needed to check to prevent cache from erasing.
        if value != self.is_sdf() {
            let ret = unsafe {
                bind::TTF_SetFontSDF(
                    self.ptr.as_ptr(),
                    if value {
                        bind::SDL_TRUE
                    } else {
                        bind::SDL_FALSE
                    },
                )
            };
            if ret == -1 {
                return Err(SdlError::UnsupportedFeature);
            }
        }
        Ok(())
    }
}
