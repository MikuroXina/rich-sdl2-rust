//! An audio stream allows to read and write by streaming method.

use std::{io, os::raw::c_int, ptr::NonNull};

use super::buffer::AudioBuffer;
use crate::{bind, Result, Sdl, SdlError};

/// An audio stream to read and write audio data by streaming method. To read/write the data, `use` the implementation [`std::io::Read`]/[`std::io::Write`] for this.
pub struct AudioStream {
    ptr: NonNull<bind::SDL_AudioStream>,
}

impl std::fmt::Debug for AudioStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioStream")
            .field("len", &self.available_bytes_len())
            .finish_non_exhaustive()
    }
}

impl AudioStream {
    /// Constructs a stream from two audio buffers.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to create a new audio stream.
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

    /// Returns the available length of the stream in bytes.
    #[must_use]
    pub fn available_bytes_len(&self) -> usize {
        unsafe { bind::SDL_AudioStreamAvailable(self.ptr.as_ptr()) as usize }
    }

    /// Clears the audio data in the stream.
    pub fn clear(&self) {
        unsafe { bind::SDL_AudioStreamClear(self.ptr.as_ptr()) }
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
                buf.as_mut_ptr().cast(),
                buf.len() as c_int,
            )
        };
        if ret < 0 {
            Err(io::Error::new(io::ErrorKind::Other, Sdl::error()))
        } else {
            Ok(ret as usize)
        }
    }
}

impl io::Write for AudioStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let ret = unsafe {
            bind::SDL_AudioStreamPut(self.ptr.as_ptr(), buf.as_ptr().cast(), buf.len() as i32)
        };
        if ret < 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let ret = unsafe { bind::SDL_AudioStreamFlush(self.ptr.as_ptr()) };
        if ret < 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(())
        }
    }
}
