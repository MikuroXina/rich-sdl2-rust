use std::ffi::CString;

use crate::bind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ButtonKind {
    Normal,
    Confirm,
    Cancel,
}

impl ButtonKind {
    #[allow(clippy::unnecessary_cast)]
    fn as_flags(self) -> u32 {
        (match self {
            ButtonKind::Normal => 0,
            ButtonKind::Confirm => bind::SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT,
            ButtonKind::Cancel => bind::SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT,
        }) as u32
    }
}

/// An id type for a button.
pub type ButtonId = i32;

/// A button in a [`super::MessageBox`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button {
    kind: ButtonKind,
    id: ButtonId,
    text: CString,
}

impl Button {
    /// Constructs a normal button.
    ///
    /// # Panics
    ///
    /// Panics if `text` contains a null character.
    #[must_use]
    pub fn normal(id: ButtonId, text: &str) -> Self {
        Self {
            kind: ButtonKind::Normal,
            id,
            text: CString::new(text).unwrap(),
        }
    }
    /// Constructs a confirm button.
    ///
    /// # Panics
    ///
    /// Panics if `text` contains a null character.
    #[must_use]
    pub fn confirm(id: ButtonId, text: &str) -> Self {
        Self {
            kind: ButtonKind::Confirm,
            id,
            text: CString::new(text).unwrap(),
        }
    }
    /// Constructs a cancel button.
    ///
    /// # Panics
    ///
    /// Panics if `text` contains a null character.
    #[must_use]
    pub fn cancel(id: ButtonId, text: &str) -> Self {
        Self {
            kind: ButtonKind::Cancel,
            id,
            text: CString::new(text).unwrap(),
        }
    }

    pub(super) fn as_raw(&self) -> bind::SDL_MessageBoxButtonData {
        bind::SDL_MessageBoxButtonData {
            flags: self.kind.as_flags(),
            buttonid: self.id,
            text: self.text.as_ptr(),
        }
    }
}
