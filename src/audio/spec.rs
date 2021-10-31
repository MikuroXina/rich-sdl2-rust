//! Provides tools to make a specification to require what an audio device is.

use bitflags::bitflags;
use std::{
    ffi::c_void,
    marker::PhantomData,
    os::raw::c_int,
    sync::{Arc, Mutex},
};
use typed_builder::TypedBuilder;

use crate::bind;

use super::format::AudioFormat;

/// A builder to build an information representing what specification is required for an audio device.
#[derive(TypedBuilder)]
pub struct AudioSpecBuilder<'callback, T: AudioCallback<'callback>> {
    #[builder(default = 44100)]
    sample_freq: u32,
    #[builder(default = AudioFormat::signed32_lsb())]
    format: AudioFormat,
    #[builder(default = 2)]
    channels: u8,
    #[builder(default = 4096)]
    samples: u16,
    #[builder(default, setter(strip_option))]
    callback: Option<T>,
    _phantom: PhantomData<&'callback mut T>,
}

/// A type of the callback to interact with the raw audio buffer.
pub trait AudioCallback<'callback>: FnMut(&mut [u8]) + 'callback {}

/// A specification to require what an audio device is.
pub struct AudioSpec<'callback, T> {
    raw: bind::SDL_AudioSpec,
    _phantom: PhantomData<&'callback mut T>,
}

impl<T> std::fmt::Debug for AudioSpec<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioSpec").finish_non_exhaustive()
    }
}

impl<'callback, T: AudioCallback<'callback>> AudioSpec<'callback, T> {
    /// Constructs an audio specification with the optional callback.
    pub fn new(builder: AudioSpecBuilder<'callback, T>) -> Self {
        Self {
            raw: bind::SDL_AudioSpec {
                freq: builder.sample_freq as c_int,
                format: builder.format.as_raw(),
                channels: builder.channels,
                silence: 0,
                samples: builder.samples,
                padding: 0,
                size: 0,
                callback: builder
                    .callback
                    .as_ref()
                    .map(|_| audio_spec_wrap_handler::<T> as _),
                userdata: builder
                    .callback
                    .map_or(std::ptr::null_mut(), |mut callback| {
                        &mut callback as *mut T as *mut _
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

unsafe extern "C" fn audio_spec_wrap_handler<'callback, T: AudioCallback<'callback>>(
    userdata: *mut c_void,
    stream: *mut u8,
    len: c_int,
) {
    if userdata.is_null() {
        return;
    }
    let func = &mut *(userdata as *mut T);
    let slice = std::slice::from_raw_parts_mut(stream, len as usize);
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
