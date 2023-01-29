//! Controls of a simple text input field.

use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::{ffi::CStr, ptr::addr_of};

use crate::geo::Rect;
use crate::{bind, Video};

/// An event on input the text directly or confirm the conversion on the window.
#[derive(Debug, Clone)]
pub struct TextInputEvent {
    /// When this event occurred.
    pub timestamp: u32,
    /// The id of the window focused.
    pub window_id: u32,
    /// The text inputted.
    pub text: String,
}

impl From<bind::SDL_TextInputEvent> for TextInputEvent {
    fn from(raw: bind::SDL_TextInputEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            text: unsafe { CStr::from_ptr(addr_of!(raw.text).cast()) }
                .to_string_lossy()
                .into(),
        }
    }
}

/// An event on editing conversion on a software input method.
#[derive(Debug, Clone)]
pub struct TextEditingEvent {
    /// When this event occurred.
    pub timestamp: u32,
    /// The id of the window focused.
    pub window_id: u32,
    /// The text inputted.
    pub text: String,
    /// The editing position from the start.
    pub start: i32,
    /// The length of editing characters.
    pub length: i32,
}

impl From<bind::SDL_TextEditingEvent> for TextEditingEvent {
    fn from(raw: bind::SDL_TextEditingEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            text: unsafe { CStr::from_ptr(addr_of!(raw.text).cast()) }
                .to_string_lossy()
                .into(),
            start: raw.start,
            length: raw.length,
        }
    }
}

/// A controller of inputting texts.
pub struct TextInput<'video> {
    video: PhantomData<&'video Video<'video>>,
}

impl std::fmt::Debug for TextInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextInput").finish()
    }
}

assert_not_impl_all!(TextInput: Send, Sync);

impl<'video> TextInput<'video> {
    /// Starts to input the text on area `input_rect`.
    #[must_use]
    pub fn new(_: &'video Video, input_rect: Rect) -> Self {
        let raw_rect = input_rect.into();
        unsafe {
            bind::SDL_SetTextInputRect(&raw_rect);
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
