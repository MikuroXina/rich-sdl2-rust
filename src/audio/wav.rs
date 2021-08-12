use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
};

use crate::{bind, Result, Sdl, SdlError};

pub struct Wav {
    buffer: *mut u8,
    len: usize,
}

impl Wav {
    pub fn new(file_name: &str) -> Result<Self> {
        let read_binary_mode = CStr::from_bytes_with_nul(b"rb\0").unwrap();
        let cstr = CString::new(file_name).expect("file_name must not be empty");
        let mut audio_spec = MaybeUninit::uninit();
        let mut buffer = std::ptr::null_mut();
        let mut len = 0u32;
        let ptr = unsafe {
            bind::SDL_LoadWAV_RW(
                bind::SDL_RWFromFile(cstr.as_ptr(), read_binary_mode.as_ptr()),
                1,
                audio_spec.as_mut_ptr(),
                &mut buffer as *mut _,
                &mut len as *mut _,
            )
        };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                buffer,
                len: len as usize,
            })
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buffer, self.len) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.buffer, self.len) }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

impl AsRef<[u8]> for Wav {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for Wav {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_slice_mut()
    }
}

impl Drop for Wav {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeWAV(self.buffer) }
    }
}