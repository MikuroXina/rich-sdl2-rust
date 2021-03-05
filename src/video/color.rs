use crate::bind;

pub mod pixel;

#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Rgba> for bind::SDL_Color {
    fn from(Rgba { r, g, b, a }: Rgba) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone)]
pub enum BlendMode {
    None,
    AlphaBlend,
    Add,
    Mul,
}

impl From<bind::SDL_BlendMode> for BlendMode {
    fn from(raw: bind::SDL_BlendMode) -> Self {
        use BlendMode::*;
        match raw {
            bind::SDL_BlendMode_SDL_BLENDMODE_BLEND => AlphaBlend,
            bind::SDL_BlendMode_SDL_BLENDMODE_ADD => Add,
            bind::SDL_BlendMode_SDL_BLENDMODE_MOD => Mul,
            _ => None,
        }
    }
}

impl From<BlendMode> for bind::SDL_BlendMode {
    fn from(raw: BlendMode) -> Self {
        use BlendMode::*;
        match raw {
            AlphaBlend => bind::SDL_BlendMode_SDL_BLENDMODE_BLEND,
            Add => bind::SDL_BlendMode_SDL_BLENDMODE_ADD,
            Mul => bind::SDL_BlendMode_SDL_BLENDMODE_MOD,
            None => bind::SDL_BlendMode_SDL_BLENDMODE_NONE,
        }
    }
}
