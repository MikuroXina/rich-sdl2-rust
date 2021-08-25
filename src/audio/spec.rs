//! Provides tools to make a specification to require what an audio device is.

use bitflags::bitflags;
use std::{
    ffi::c_void,
    marker::PhantomData,
    os::raw::c_int,
    sync::{Arc, Mutex},
};

use crate::bind;

use super::format::AudioFormat;

/// A builder to build an information representing what specification is required for an audio device.
#[derive(Debug, Clone)]
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
    /// Constructs an empty builder with the standard specification.
    pub fn new() -> Self {
        Self {
            sample_freq: 44100,
            format: AudioFormat::signed32_lsb(),
            channels: 2,
            samples: 4096,
        }
    }

    /// Changes the sample frequencies of the specification.
    pub fn sample_freq(&mut self, value: u32) -> &mut Self {
        self.sample_freq = value;
        self
    }

    /// Changes the format of the specification.
    pub fn format(&mut self, value: AudioFormat) -> &mut Self {
        self.format = value;
        self
    }

    /// Changes the numbers of channels of the specification.
    pub fn channels(&mut self, value: u8) -> &mut Self {
        self.channels = value;
        self
    }

    /// Changes the sample rates of the specification.
    pub fn samples(&mut self, value: u16) -> &mut Self {
        self.samples = value;
        self
    }

    /// Builds an [`AudioSpec`] with an optional callback.
    pub fn build<'callback>(
        self,
        callback: Option<Box<dyn AudioCallback + 'callback>>,
    ) -> AudioSpec<'callback> {
        AudioSpec::new(self, callback.map(Box::new))
    }
}

/// A type of the callback to interact with the raw audio buffer.
pub trait AudioCallback: FnMut(&mut [u8]) {}

/// A specification to require what an audio device is.
pub struct AudioSpec<'callback> {
    raw: bind::SDL_AudioSpec,
    _phantom: PhantomData<&'callback mut dyn AudioCallback>,
}

impl std::fmt::Debug for AudioSpec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioSpec").finish()
    }
}

impl<'callback> AudioSpec<'callback> {
    fn new(
        builder: AudioSpecBuilder,
        callback: Option<Box<Box<dyn AudioCallback + 'callback>>>,
    ) -> Self {
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
                userdata: callback.map_or(std::ptr::null_mut(), |callback| {
                    Box::into_raw(callback).cast()
                }),
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

bitflags! {
    /// A flag what component may fallback into an actual audio device.
    pub struct FallbackFlag : u32 {
        /// Allows to fallback frequencies.
        const FREQUENCY = 1 << 0;
        /// Allows to fallback a format.
        const FORMAT = 1 << 0;
        /// Allows to fallback numbers of channels.
        const CHANNELS = 1 << 0;
        /// Allows to fallback sample rates.
        const SAMPLES = 1 << 0;
    }
}
