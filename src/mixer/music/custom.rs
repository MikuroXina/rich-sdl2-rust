//! A music made from the custom callback function.

use std::{ffi::c_void, os::raw::c_int};

use super::MixMusic;
use crate::bind;

/// A callback to generate the audio to play.
pub type MusicCallback<'device> = Box<dyn FnMut(&mut [u8]) + 'device>;

/// A custom music made by the callback.
pub struct MixCustomMusic<'device> {
    raw: *mut MusicCallback<'device>,
}

impl<'device> MixCustomMusic<'device> {
    /// Constructs a custom music and starts immediately.
    pub fn new_and_play(
        _device: &'device MixMusic<'device>,
        callback: MusicCallback<'device>,
    ) -> Self {
        let wrapped = Box::new(callback);
        let raw = Box::into_raw(wrapped);
        unsafe { bind::Mix_HookMusic(Some(mix_custom_music_handler), raw.cast()) }
        Self { raw }
    }
}

extern "C" fn mix_custom_music_handler(userdata: *mut c_void, stream: *mut u8, len: c_int) {
    let callback = unsafe { &mut *(userdata as *mut MusicCallback) };
    let stream = unsafe { std::slice::from_raw_parts_mut(stream, len as _) };
    callback(stream);
}

impl Drop for MixCustomMusic<'_> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.raw) };
        unsafe { bind::Mix_HookMusic(None, std::ptr::null_mut()) }
    }
}
