use std::{io, os::raw::c_int, ptr::NonNull};

use super::buffer::AudioBuffer;
use crate::{bind, Result, Sdl, SdlError};

pub struct AudioStream {
    ptr: NonNull<bind::SDL_AudioStream>,
}

impl AudioStream {
    pub fn new<T, U>(src: AudioBuffer<T>, dst: AudioBuffer<U>) -> Result<Self> {
        let ptr = unsafe {
            bind::SDL_NewAudioStream(
                src.format().as_raw(),
                src.channels(),
                src.samples() as i32,
                dst.format().as_raw(),
                dst.channels(),
                dst.samples() as i32,
            )
        };
        if ptr.is_null() {
            let msg = Sdl::error();
            Err(if msg == "Out of memory" {
                SdlError::OutOfMemory
            } else {
                SdlError::Others { msg }
            })
        } else {
            Ok(Self {
                ptr: NonNull::new(ptr).unwrap(),
            })
        }
    }

    pub fn clear(&self) {
        unsafe { bind::SDL_AudioStreamClear(self.ptr.as_ptr()) }
    }

    pub fn flush(&self) -> Result<()> {
        let ret = unsafe { bind::SDL_AudioStreamFlush(self.ptr.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }
}

impl Drop for AudioStream {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeAudioStream(self.ptr.as_ptr()) }
    }
}

impl io::Read for AudioStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ret = unsafe {
            bind::SDL_AudioStreamGet(
                self.ptr.as_ptr(),
                buf.as_mut_ptr() as *mut _,
                buf.len() as c_int,
            )
        };
        if ret < 0 {
            Err(io::Error::new(io::ErrorKind::InvalidInput, Sdl::error()))
        } else {
            Ok(ret as usize)
        }
    }
}
