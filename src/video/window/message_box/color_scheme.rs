use crate::{bind, color::Rgb};

/// A color scheme for a message box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorScheme {
    /// A background color in the message box.
    pub background: Rgb,
    /// A foreground text color in the message box.
    pub text: Rgb,
    /// A border color in the message box button.
    pub button_border: Rgb,
    /// A background color in the message box button.
    pub button_background: Rgb,
    /// A selected color in the message box button.
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
