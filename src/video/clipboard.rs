//! Clipboard controls.

use std::ffi::CString;

use crate::bind;

/// A text from the clipboard.
#[derive(Debug, PartialEq, Eq)]
pub struct ClipboardText {
    text: String,
}

impl ClipboardText {
    /// Get a clipboard text if exists.
    pub fn new() -> Option<Self> {
        let ptr = unsafe { bind::SDL_GetClipboardText() };
        if ptr.is_null() {
            return None;
        }
        let text = unsafe { CString::from_raw(ptr) }.into_string().ok()?;
        unsafe { bind::SDL_free(ptr.cast()) }
        Some(Self { text })
    }

    /// Returns a reference to the clipboard string .
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Converts into `String`.
    pub fn into_string(self) -> String {
        self.text
    }
}
