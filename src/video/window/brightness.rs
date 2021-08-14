use crate::gamma_ramp::GammaRamp;
use crate::{bind, Result, Sdl, SdlError};

use super::Window;

#[derive(Debug)]
pub struct Brightness {
    brightness: f32,
}

impl Brightness {
    pub fn new(brightness: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&brightness) {
            Some(Self { brightness })
        } else {
            None
        }
    }

    pub fn as_f32(&self) -> f32 {
        self.brightness
    }
}

pub trait BrightnessExt {
    fn brightness(&self) -> Brightness;
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

#[derive(Debug, Default)]
pub struct Gamma {
    pub red: GammaRamp,
    pub green: GammaRamp,
    pub blue: GammaRamp,
}

#[derive(Debug)]
pub struct GammaParam {
    pub red: Option<GammaRamp>,
    pub green: Option<GammaRamp>,
    pub blue: Option<GammaRamp>,
}

pub trait GammaExt {
    fn gamma(&self) -> Result<Gamma>;
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
