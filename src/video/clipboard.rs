use std::ffi::CString;

use crate::bind;

pub struct ClipboardText {
    text: String,
}

impl ClipboardText {
    pub fn new() -> Option<Self> {
        let ptr = unsafe { bind::SDL_GetClipboardText() };
        if ptr == std::ptr::null_mut() {
            return None;
        }
        let text = unsafe { CString::from_raw(ptr) }.into_string().ok()?;
        unsafe { bind::SDL_free(ptr as *mut _) }
        Some(Self { text })
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}
