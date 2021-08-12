use std::ffi::CStr;

use crate::bind;

pub fn all_audio_drivers() -> Vec<String> {
    let num = unsafe { bind::SDL_GetNumAudioDrivers() };
    (0..num)
        .map(|index| {
            let cstr = unsafe { CStr::from_ptr(bind::SDL_GetAudioDriver(index)) };
            cstr.to_string_lossy().to_string()
        })
        .collect()
}

pub fn current_driver() -> Option<String> {
    let ptr = unsafe { bind::SDL_GetCurrentAudioDriver() };
    (!ptr.is_null()).then(|| {
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_string_lossy().to_string()
    })
}
