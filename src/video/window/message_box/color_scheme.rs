use crate::{bind, color::Rgb};

#[derive(Debug)]
pub struct ColorScheme {
    pub background: Rgb,
    pub text: Rgb,
    pub button_border: Rgb,
    pub button_background: Rgb,
    pub button_selected: Rgb,
}

impl From<Rgb> for bind::SDL_MessageBoxColor {
    fn from(Rgb { r, g, b }: Rgb) -> Self {
        Self { r, g, b }
    }
}

impl From<ColorScheme> for bind::SDL_MessageBoxColorScheme {
    fn from(scheme: ColorScheme) -> Self {
        Self {
            colors: [
                scheme.background.into(),
                scheme.text.into(),
                scheme.button_border.into(),
                scheme.button_background.into(),
                scheme.button_selected.into(),
            ],
        }
    }
}
