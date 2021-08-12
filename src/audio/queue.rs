use crate::{bind, Result, Sdl, SdlError};

use super::SpeakerDevice;

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
            unsafe { bind::SDL_QueueAudio(self.device.id, data.as_ptr() as *const _, size as u32) };
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
