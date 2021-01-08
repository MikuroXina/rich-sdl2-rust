use crate::{bind, Result, Sdl, SdlError};

use super::Window;

#[derive(Debug)]
pub struct Brightness {
    brightness: f32,
}

impl Brightness {
    pub fn new(brightness: f32) -> Option<Self> {
        if 0.0 <= brightness && brightness <= 1.0 {
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
            return Err(SdlError::UnsupportedFeature { msg: Sdl::error() });
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct GammaRamp(Box<[u16; 256]>);

impl Default for GammaRamp {
    fn default() -> Self {
        Self(Box::new([0; 256]))
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
                gamma.red.0.as_mut_ptr() as *mut _,
                gamma.green.0.as_mut_ptr() as *mut _,
                gamma.blue.0.as_mut_ptr() as *mut _,
            )
        };
        if ret != 0 {
            let msg = Sdl::error();
            return Err(if msg == "Out of memory" {
                SdlError::OutOfMemory
            } else {
                SdlError::UnsupportedFeature { msg }
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
                    .map_or(std::ptr::null(), |ramp| ramp.0.as_ptr() as *const _),
                gamma
                    .green
                    .map_or(std::ptr::null(), |ramp| ramp.0.as_ptr() as *const _),
                gamma
                    .blue
                    .map_or(std::ptr::null(), |ramp| ramp.0.as_ptr() as *const _),
            )
        };
        if ret != 0 {
            let msg = Sdl::error();
            return Err(if msg == "Out of memory" {
                SdlError::OutOfMemory
            } else {
                SdlError::UnsupportedFeature { msg }
            });
        }
        Ok(())
    }
}
