use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

use crate::geo::Rect;
use crate::{bind, Video};

#[derive(Debug, Clone)]
pub struct TextInputEvent {
    pub timestamp: u32,
    pub window_id: u32,
    pub text: String,
}

impl From<bind::SDL_TextInputEvent> for TextInputEvent {
    fn from(raw: bind::SDL_TextInputEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            text: unsafe { CStr::from_ptr(&raw.text as *const c_char) }
                .to_string_lossy()
                .into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextEditingEvent {
    pub timestamp: u32,
    pub window_id: u32,
    pub text: String,
    pub start: i32,
    pub length: i32,
}

impl From<bind::SDL_TextEditingEvent> for TextEditingEvent {
    fn from(raw: bind::SDL_TextEditingEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            text: unsafe { CStr::from_ptr(&raw.text as *const c_char) }
                .to_string_lossy()
                .into(),
            start: raw.start,
            length: raw.length,
        }
    }
}

pub struct TextInput<'video> {
    video: PhantomData<&'video ()>,
}

impl<'video> TextInput<'video> {
    pub fn new(_: &'video Video, input_rect: Rect) -> Self {
        let mut raw_rect = input_rect.into();
        unsafe {
            bind::SDL_SetTextInputRect(&mut raw_rect as *mut _);
            bind::SDL_StartTextInput();
        }
        Self { video: PhantomData }
    }
}

impl Drop for TextInput<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_StopTextInput() }
    }
}
