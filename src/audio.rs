use std::ffi::CStr;

use crate::bind;

pub mod format;
pub mod spec;

pub struct AudioDevice {
    name: String,
}

impl AudioDevice {
    pub fn all_devices() -> impl Iterator<Item = Self> {
        devices(false).map(|name| Self { name })
    }
}

pub struct CaptureAudioDevice {
    name: String,
}

impl CaptureAudioDevice {
    pub fn all_devices() -> impl Iterator<Item = Self> {
        devices(true).map(|name| Self { name })
    }
}

fn devices(is_capture: bool) -> impl Iterator<Item = String> {
    let is_capture_raw = if is_capture { 1 } else { 0 };
    let num_devices = unsafe {
        bind::SDL_InitSubSystem(bind::SDL_INIT_AUDIO);
        bind::SDL_GetNumAudioDevices(is_capture_raw)
    };
    (0..num_devices).map(move |index| {
        let cstr = unsafe { CStr::from_ptr(bind::SDL_GetAudioDeviceName(index, is_capture_raw)) };
        cstr.to_string_lossy().to_string()
    })
}
