use std::{
    ffi::c_void,
    marker::PhantomData,
    os::raw::c_int,
    sync::{Arc, Mutex},
};

use crate::bind;

use super::format::AudioFormat;

#[derive(Debug)]
pub struct AudioSpecBuilder {
    sample_freq: u32,
    format: AudioFormat,
    channels: u8,
    samples: u16,
}

impl Default for AudioSpecBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioSpecBuilder {
    pub fn new() -> Self {
        Self {
            sample_freq: 44100,
            format: AudioFormat::signed32_lsb(),
            channels: 2,
            samples: 4096,
        }
    }

    pub fn sample_freq(&mut self, value: u32) -> &mut Self {
        self.sample_freq = value;
        self
    }

    pub fn format(&mut self, value: AudioFormat) -> &mut Self {
        self.format = value;
        self
    }

    pub fn channels(&mut self, value: u8) -> &mut Self {
        self.channels = value;
        self
    }

    pub fn samples(&mut self, value: u16) -> &mut Self {
        self.samples = value;
        self
    }

    pub fn build<'callback>(
        self,
        callback: impl AudioCallback + 'callback,
    ) -> AudioSpec<'callback> {
        AudioSpec::new(self, Box::new(Box::new(callback)))
    }
}

pub trait AudioCallback: FnMut(&mut [u8]) {}

pub struct AudioSpec<'callback> {
    raw: bind::SDL_AudioSpec,
    _phantom: PhantomData<&'callback mut dyn AudioCallback>,
}

impl<'callback> AudioSpec<'callback> {
    fn new(builder: AudioSpecBuilder, callback: Box<Box<dyn AudioCallback + 'callback>>) -> Self {
        Self {
            raw: bind::SDL_AudioSpec {
                freq: builder.sample_freq as c_int,
                format: builder.format.as_raw(),
                channels: builder.channels,
                silence: 0,
                samples: builder.samples,
                padding: 0,
                size: 0,
                callback: Some(audio_spec_wrap_handler),
                userdata: Box::into_raw(callback) as *mut _,
            },
            _phantom: PhantomData,
        }
    }

    pub(super) fn raw(&self) -> &bind::SDL_AudioSpec {
        &self.raw
    }

    pub(super) fn raw_mut(&mut self) -> &mut bind::SDL_AudioSpec {
        &mut self.raw
    }
}

impl Drop for AudioSpec<'_> {
    fn drop(&mut self) {
        let func = unsafe { self.raw.userdata as *mut Box<dyn AudioCallback> };
        let _ = unsafe { Box::from_raw(func) };
    }
}

extern "C" fn audio_spec_wrap_handler(userdata: *mut c_void, stream: *mut u8, len: c_int) {
    let func = unsafe { &mut *(userdata as *mut Box<dyn AudioCallback>) };
    let slice = unsafe { std::slice::from_raw_parts_mut(stream, len as usize) };
    slice.fill(0);
    func(slice);
}
