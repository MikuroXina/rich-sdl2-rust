use crate::bind;

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
