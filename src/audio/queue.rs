//! Structures to use an audio device by pushing methods.

use std::io;

use super::{MicrophoneDevice, SpeakerDevice};
use crate::{bind, Result, Sdl, SdlError};

/// A queue to push data to play the sound with [`SpeakerDevice`].
#[derive(Debug)]
pub struct QueuedAudio<'device> {
    device: &'device mut SpeakerDevice,
}

impl<'device> QueuedAudio<'device> {
    /// Constructs a queue from a [`SpeakerDevice`].
    pub fn new(device: &'device mut SpeakerDevice) -> Self {
        Self { device }
    }

    /// Queues the `data` to play.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to queue `data`.
    pub fn queue<T>(&self, data: &[T]) -> Result<()> {
        let size = data.len() * std::mem::size_of::<T>();
        let ret =
            unsafe { bind::SDL_QueueAudio(self.device.id, data.as_ptr().cast(), size as u32) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Clears all audio data in the queue to stop to play, but it does not stop immediately because of drivers' implementation.
    pub fn clear(&self) {
        unsafe { bind::SDL_ClearQueuedAudio(self.device.id) }
    }

    /// Returns the size of the queue in bytes.
    #[must_use]
    pub fn queue_bytes_size(&self) -> usize {
        unsafe { bind::SDL_GetQueuedAudioSize(self.device.id) as usize }
    }
}

impl Drop for QueuedAudio<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_ClearQueuedAudio(self.device.id) }
    }
}

/// A queue to read data to record the sound with [`MicrophoneDevice`]. To dequeue from this, `use` the implementation of [`std::io::Read`] for this.
#[derive(Debug)]
pub struct DequeueAudio<'device> {
    device: &'device mut MicrophoneDevice,
}

impl<'device> DequeueAudio<'device> {
    /// Constructs a queue from [`MicrophoneDevice`].
    pub fn new(device: &'device mut MicrophoneDevice) -> Self {
        Self { device }
    }

    /// Clears all audio data in the queue to prevent queue from unnecessary data.
    pub fn clear(&self) {
        unsafe { bind::SDL_ClearQueuedAudio(self.device.id) }
    }
}

impl Drop for DequeueAudio<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_ClearQueuedAudio(self.device.id) }
    }
}

impl io::Read for DequeueAudio<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes = unsafe {
            bind::SDL_DequeueAudio(self.device.id, buf.as_mut_ptr().cast(), buf.len() as u32)
        };
        Ok(bytes as usize)
    }
}
