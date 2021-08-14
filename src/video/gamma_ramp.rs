use crate::bind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GammaRamp(pub Box<[u16; 256]>);

impl GammaRamp {
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
