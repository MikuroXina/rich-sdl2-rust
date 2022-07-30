//! Audio specifications for SDL2_mixer.

use std::{borrow::Cow, ffi::CStr, marker::PhantomData};

use crate::{
    audio::format::AudioFormat,
    bind,
    mixer::{
        chunk::channel::{
            effect::{effect_attach_done_handler, effect_attach_effect_handler, Effect},
            ChannelGroup,
        },
        Mix,
    },
    Result, Sdl, SdlError,
};

/// A builder for [`MixDevice`].
#[derive(Debug, Clone)]
pub struct MixDeviceBuilder {
    frequency: u32,
    format: AudioFormat,
    channels: u32,
    chunk_size: u32,
}

impl MixDeviceBuilder {
    /// Constructs a default builder.
    pub fn new() -> Self {
        Self {
            frequency: bind::MIX_DEFAULT_FREQUENCY,
            format: if cfg!(target_endian = "big") {
                AudioFormat::signed16_msb()
            } else {
                AudioFormat::signed16_lsb()
            },
            channels: 2,
            chunk_size: bind::MIX_DEFAULT_CHANNELS,
        }
    }

    /// Changes the sampling frequencies.
    pub fn frequency(&mut self, frequency: u32) -> &mut Self {
        self.frequency = frequency;
        self
    }

    /// Changes the audio format.
    pub fn format(&mut self, format: AudioFormat) -> &mut Self {
        self.format = format;
        self
    }

    /// Changes the numbers of channels.
    pub fn channels(&mut self, channels: u32) -> &mut Self {
        self.channels = channels;
        self
    }

    /// Changes the output chunk size. If it is too low, the sound may skip.
    pub fn chunk_size(&mut self, chunk_size: u32) -> &mut Self {
        self.chunk_size = chunk_size;
        self
    }

    /// Opens a [`MixDevice`] with a root controller, or `Err` on failure.
    pub fn build(self, _mix: &Mix) -> Result<MixDevice> {
        let format = (self.format.flag.bits() as u16) << 8 | self.format.bit_size as u16;
        let ret = unsafe {
            bind::Mix_OpenAudio(
                self.frequency as _,
                format,
                self.channels as _,
                self.chunk_size as _,
            )
        };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(MixDevice {
                _phantom: PhantomData,
            })
        }
    }
}

impl Default for MixDeviceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A specification of the [`MixDevice`], returned from [`MixDevice::query`].
#[derive(Debug, Clone, Copy)]
pub struct MixSpec {
    /// The sampling frequency of the audio device.
    pub frequency: u32,
    /// The output format of the audio device.
    pub format: AudioFormat,
    /// The numbers of channels of the audio device.
    pub channels: u32,
}

/// An audio device built from [`MixDeviceBuilder`].
pub struct MixDevice<'mix> {
    _phantom: PhantomData<&'mix Mix>,
}

impl MixDevice<'_> {
    /// Queries the specification of the audio device.
    pub fn query(&self) -> MixSpec {
        let mut frequency = 0;
        let mut format = 0;
        let mut channels = 0;
        let _ = unsafe {
            bind::Mix_QuerySpec(
                &mut frequency as *mut _,
                &mut format as *mut _,
                &mut channels as *mut _,
            )
        };
        MixSpec {
            frequency: frequency as _,
            format: format.into(),
            channels: channels as _,
        }
    }

    /// Returns the decoder names for the mix chunk.
    pub fn chunk_decoders(&self) -> Vec<Cow<str>> {
        let num = unsafe { bind::Mix_GetNumChunkDecoders() };
        (0..num)
            .map(|index| {
                let cstr = unsafe { CStr::from_ptr(bind::Mix_GetChunkDecoder(index)) };
                cstr.to_string_lossy()
            })
            .collect()
    }

    /// Returns the decoder names for the mix music.
    pub fn music_decoders(&self) -> Vec<Cow<str>> {
        let num = unsafe { bind::Mix_GetNumMusicDecoders() };
        (0..num)
            .map(|index| {
                let cstr = unsafe { CStr::from_ptr(bind::Mix_GetMusicDecoder(index)) };
                cstr.to_string_lossy()
            })
            .collect()
    }

    /// Returns the numbers of playing channels.
    pub fn playing_channels(&self) -> usize {
        unsafe { bind::Mix_Playing(-1) as _ }
    }

    /// Constructs the channel group.
    pub fn new_channels(&self, len: usize) -> ChannelGroup {
        ChannelGroup::new(self, len)
    }

    /// Attaches the effect to the special, post effect channel.
    pub fn attach_post_effect(&self, effect: Effect) {
        let wrapped = Box::new(effect);
        let raw = Box::into_raw(wrapped);
        let _ = unsafe {
            bind::Mix_RegisterEffect(
                bind::MIX_CHANNEL_POST,
                Some(effect_attach_effect_handler),
                Some(effect_attach_done_handler),
                raw.cast(),
            )
        };
    }

    /// Detaches all the effect from the special, post effect channel.
    pub fn detach_all_post_effect(&self) {
        let _ = unsafe { bind::Mix_UnregisterAllEffects(bind::MIX_CHANNEL_POST) };
    }
}

impl Drop for MixDevice<'_> {
    fn drop(&mut self) {
        unsafe { bind::Mix_CloseAudio() }
    }
}
