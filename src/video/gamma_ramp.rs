//! A gamma ramp for display color management.

use crate::bind;

/// A gamma ramp for a display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GammaRamp(pub Box<[u16; 256]>);

impl GammaRamp {
    /// Constructs a gamma ramp from the gamma value.
    pub fn new(gamma: f32) -> Self {
        let mut ramp = Self::default();
        unsafe { bind::SDL_CalculateGammaRamp(gamma, ramp.0.as_mut_ptr()) }
        ramp
    }
}

impl Default for GammaRamp {
    fn default() -> Self {
        Self(Box::new([0; 256]))
    }
}
