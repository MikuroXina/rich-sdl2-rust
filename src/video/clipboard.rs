use std::ffi::CString;

use crate::bind;

#[derive(Debug, PartialEq, Eq)]
pub struct ClipboardText {
    text: String,
}

impl ClipboardText {
    pub fn new() -> Option<Self> {
        let ptr = unsafe { bind::SDL_GetClipboardText() };
        if ptr.is_null() {
            return None;
        }
        let text = unsafe { CString::from_raw(ptr) }.into_string().ok()?;
        unsafe { bind::SDL_free(ptr.cast()) }
        Some(Self { text })
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn into_string(self) -> String {
        self.text
    }
}
