//! Modal control for a window.

use crate::{as_raw, bind, Result, Sdl, SdlError};
use std::ffi::CString;

use super::Window;

pub use self::{
    button::{Button, ButtonId},
    color_scheme::ColorScheme,
};

mod button;
mod color_scheme;

/// A kind of [`MessageBox`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MessageBoxKind {
    /// An error message.
    Error,
    /// A warning message.
    Warning,
    /// An information message.
    Information,
}

/// A message box builder.
#[derive(Debug, Clone)]
pub struct MessageBox {
    kind: MessageBoxKind,
    title: CString,
    message: CString,
    buttons: Vec<Button>,
    color_scheme: Option<ColorScheme>,
}

impl MessageBox {
    /// Constructs a message box builder from the kind.
    #[must_use]
    pub fn new(kind: MessageBoxKind) -> Self {
        Self {
            kind,
            title: CString::default(),
            message: CString::default(),
            buttons: vec![],
            color_scheme: None,
        }
    }

    /// Sets the title of the message box.
    ///
    /// # Panics
    ///
    /// Panics if `title` contains a null character.
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = CString::new(title).unwrap();
        self
    }

    /// Sets the message of the message box.
    ///
    /// # Panics
    ///
    /// Panics if `message` contains a null character.
    pub fn message(&mut self, message: &str) -> &mut Self {
        self.message = CString::new(message).unwrap();
        self
    }

    /// Adds a button to the message box.
    pub fn add_button(&mut self, button: Button) -> &mut Self {
        self.buttons.push(button);
        self
    }

    /// Sets the color scheme of the message box.
    pub fn color_scheme(&mut self, scheme: Option<ColorScheme>) -> &mut Self {
        self.color_scheme = scheme;
        self
    }

    /// Shows the message box for a window. And returns the pushed button's [`ButtonId`].
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to show the message box.
    pub fn show(self, parent: &'_ Window<'_>) -> Result<ButtonId> {
        let title_cstr = CString::new(self.title).unwrap_or_default();
        let message_cstr = CString::new(self.message).unwrap_or_default();
        let buttons_raw: Vec<_> = self.buttons.iter().map(Button::as_raw).collect();
        let color_scheme = self.color_scheme.map(Into::into);
        let data = bind::SDL_MessageBoxData {
            flags: 0,
            window: parent.as_ptr(),
            title: title_cstr.as_ptr(),
            message: message_cstr.as_ptr(),
            numbuttons: buttons_raw.len() as i32,
            buttons: buttons_raw.as_ptr(),
            colorScheme: unsafe { as_raw(&color_scheme) },
        };
        let mut button_id = 0;
        let ret = unsafe { bind::SDL_ShowMessageBox(&data, &mut button_id) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(button_id)
    }
}
