#[derive(Debug)]
pub struct GammaRamp(pub Box<[u16; 256]>);

impl Default for GammaRamp {
    fn default() -> Self {
        Self(Box::new([0; 256]))
    }
}
