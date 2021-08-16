use crate::gamma_ramp::GammaRamp;
use crate::{bind, Result, Sdl, SdlError};

use super::Window;

/// A brightness in the window.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Brightness {
    brightness: f32,
}

impl Brightness {
    /// Constructs from brightness, or `None` if the value is not in `0.0..=1.0`.
    pub fn new(brightness: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&brightness) {
            Some(Self { brightness })
        } else {
            None
        }
    }

    /// Constructs from brightness, clamping to `0.0,,=1.0`.
    pub fn with_clamped(brightness: f32) -> Self {
        Self {
            brightness: brightness.clamp(0.0, 1.0),
        }
    }

    /// Converts into `f32`.
    pub fn as_f32(self) -> f32 {
        self.brightness
    }
}

/// An extension for [`Window`] to get/set the brightness.
pub trait BrightnessExt {
    /// Returns the brightness of the window.
    fn brightness(&self) -> Brightness;
    /// Sets the brightness of the Window. Returns `Err` on failure.
    fn set_brightness(&self, brightness: Brightness) -> Result<()>;
}

impl BrightnessExt for Window<'_> {
    fn brightness(&self) -> Brightness {
        let brightness = unsafe { bind::SDL_GetWindowBrightness(self.as_ptr()) };
        Brightness { brightness }
    }

    fn set_brightness(&self, brightness: Brightness) -> Result<()> {
        let ret = unsafe { bind::SDL_SetWindowBrightness(self.as_ptr(), brightness.as_f32()) };
        if ret != 0 {
            return Err(SdlError::UnsupportedFeature);
        }
        Ok(())
    }
}

/// A gamma ramps for a window.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Gamma {
    /// A gamma ramp of red component.
    pub red: GammaRamp,
    /// A gamma ramp of green component.
    pub green: GammaRamp,
    /// A gamma ramp of blue component.
    pub blue: GammaRamp,
}

/// A gamma ramps for setting to a window.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GammaParam {
    /// A gamma ramp of red component. It will not set if `None`.
    pub red: Option<GammaRamp>,
    /// A gamma ramp of green component. It will not set if `None`.
    pub green: Option<GammaRamp>,
    /// A gamma ramp of blue component. It will not set if `None`.
    pub blue: Option<GammaRamp>,
}

/// An extension for [`Window`] to get/set the gamma ramp.
pub trait GammaExt {
    /// Returns the gamma ramps of the window.
    fn gamma(&self) -> Result<Gamma>;
    /// Sets the gamma ramps of the window. Returns `Err` on failure.
    fn set_gamma(&self, gamma: GammaParam) -> Result<()>;
}

impl GammaExt for Window<'_> {
    fn gamma(&self) -> Result<Gamma> {
        let mut gamma = Gamma::default();
        let ret = unsafe {
            bind::SDL_GetWindowGammaRamp(
                self.as_ptr(),
                gamma.red.0.as_mut_ptr().cast(),
                gamma.green.0.as_mut_ptr().cast(),
                gamma.blue.0.as_mut_ptr().cast(),
            )
        };
        if ret != 0 {
            let msg = Sdl::error();
            return Err(if msg == "Out of memory" {
                SdlError::OutOfMemory
            } else {
                SdlError::UnsupportedFeature
            });
        }
        Ok(gamma)
    }

    fn set_gamma(&self, gamma: GammaParam) -> Result<()> {
        let ret = unsafe {
            bind::SDL_SetWindowGammaRamp(
                self.as_ptr(),
                gamma
                    .red
                    .map_or(std::ptr::null(), |ramp| ramp.0.as_ptr().cast()),
                gamma
                    .green
                    .map_or(std::ptr::null(), |ramp| ramp.0.as_ptr().cast()),
                gamma
                    .blue
                    .map_or(std::ptr::null(), |ramp| ramp.0.as_ptr().cast()),
            )
        };
        if ret != 0 {
            let msg = Sdl::error();
            return Err(if msg == "Out of memory" {
                SdlError::OutOfMemory
            } else {
                SdlError::UnsupportedFeature
            });
        }
        Ok(())
    }
}
