use std::io;

use super::{MicrophoneDevice, SpeakerDevice};
use crate::{bind, Result, Sdl, SdlError};

pub struct QueuedAudio<'device> {
    device: &'device mut SpeakerDevice,
}

impl<'device> QueuedAudio<'device> {
    pub fn new(device: &'device mut SpeakerDevice) -> Self {
        Self { device }
    }

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

    pub fn clear(&self) {
        unsafe { bind::SDL_ClearQueuedAudio(self.device.id) }
    }

    pub fn queue_bytes_size(&self) -> usize {
        unsafe { bind::SDL_GetQueuedAudioSize(self.device.id) as usize }
    }
}

impl Drop for QueuedAudio<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_ClearQueuedAudio(self.device.id) }
    }
}

pub struct DequeueAudio<'device> {
    device: &'device mut MicrophoneDevice,
}

impl<'device> DequeueAudio<'device> {
    pub fn new(device: &'device mut MicrophoneDevice) -> Self {
        Self { device }
    }

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
