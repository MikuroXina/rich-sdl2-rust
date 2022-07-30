//! An audio buffer `AudioBuffer<T>` with a format, sample rates, numbers of channels and a buffer.
//! It can be convert into another format and mix by the specified volume.

use std::{mem::MaybeUninit, os::raw::c_int};

use super::format::AudioFormat;
use crate::{
    bind::{self, SDL_MixAudioFormat},
    Result, Sdl, SdlError,
};

/// An audio buffer with a format, sample rates, numbers of channels and a buffer.
#[derive(Debug, Clone)]
pub struct AudioBuffer<T> {
    format: AudioFormat,
    samples: u32,
    channels: u8,
    buffer: Vec<T>,
}

impl<T> AudioBuffer<T> {
    /// Constructs an audio buffer from arguments.
    /// The size of type which stored by buffer must equal to the format bit size.
    ///
    /// # Panics
    ///
    /// Panics if the size of type `T` does not equal to the format bit size.
    #[must_use]
    pub fn new(format: AudioFormat, samples: u32, channels: u8, buffer: Vec<T>) -> Self {
        assert_eq!(format.bit_size as usize, std::mem::size_of::<T>() * 8);
        Self {
            format,
            samples,
            channels,
            buffer,
        }
    }

    /// Returns the format of the audio buffer.
    pub fn format(&self) -> &AudioFormat {
        &self.format
    }

    /// Returns the sample rates of the audio buffer.
    #[must_use]
    pub fn samples(&self) -> u32 {
        self.samples
    }

    /// Returns the numbers of channels of the audio buffer.
    #[must_use]
    pub fn channels(&self) -> u8 {
        self.channels
    }

    /// Convert into another `AudioBuffer` with different format, sample rate or channels.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to convert into a specific format.
    pub fn convert<U: Default + Clone>(
        self,
        format: AudioFormat,
        samples: u32,
        channels: u8,
    ) -> Result<AudioBuffer<U>> {
        let mut dst = AudioBuffer::<U> {
            format,
            samples,
            channels,
            buffer: vec![],
        };
        self.convert_in(&mut dst)?;
        Ok(dst)
    }

    /// Convert and write into another existing `AudioBuffer`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to convert between `self` and `other`.
    pub fn convert_in<U: Default + Clone>(self, other: &mut AudioBuffer<U>) -> Result<()> {
        let mut cvt = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDL_BuildAudioCVT(
                cvt.as_mut_ptr(),
                self.format.as_raw(),
                self.channels,
                self.samples as c_int,
                other.format.as_raw(),
                other.channels,
                other.samples as c_int,
            )
        };
        if ret == 0 {
            return Err(SdlError::UnsupportedFeature);
        }
        if ret < 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        let mut cvt = unsafe { cvt.assume_init() };
        if cvt.needed == 0 {
            return Err(SdlError::UnsupportedFeature);
        }
        let len = self.samples as usize * self.channels as usize * std::mem::size_of::<T>();
        cvt.len = len as c_int;
        other.buffer.clear();
        other
            .buffer
            .resize(len * cvt.len_mult as usize, U::default());
        cvt.buf = as_u8_slice_mut(&mut other.buffer).as_mut_ptr();
        let ret = unsafe { bind::SDL_ConvertAudio(&mut cvt) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }
}

impl<T: Default + Clone> AudioBuffer<T> {
    /// Mix into another `AudioBuffer` with the specified `volume`.
    ///
    /// The max value of `volume` is `128`, saturating if it is over the max.
    #[must_use]
    pub fn mix(&self, volume: u8) -> Self {
        let mut dst = self.clone();
        self.mix_in(&mut dst, volume).unwrap();
        dst
    }

    /// Mix into another existing `AudioBuffer` with the specified `volume`.
    ///
    /// The max value of `volume` is `128`, saturating if it is over the max.
    ///
    /// # Errors
    ///
    /// Return `Err` if the formats of `self` and `dst` are different.
    pub fn mix_in(&self, dst: &mut Self, volume: u8) -> Result<()> {
        if self.format.as_raw() != dst.format.as_raw() {
            return Err(SdlError::Others {
                msg: "Cannot mix in buffers which have different format".into(),
            });
        }
        dst.buffer.clear();
        dst.buffer.resize(self.buffer.len(), T::default());
        let len = self.buffer.len() * std::mem::size_of::<T>();
        unsafe {
            bind::SDL_MixAudioFormat(
                as_u8_slice_mut(&mut dst.buffer).as_mut_ptr().cast(),
                as_u8_slice(&self.buffer).as_ptr().cast(),
                self.format.as_raw(),
                len as u32,
                volume.min(bind::SDL_MIX_MAXVOLUME as u8) as c_int,
            );
        }
        Ok(())
    }
}

fn as_u8_slice<T>(slice: &[T]) -> &[u8] {
    let size = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * size) }
}

fn as_u8_slice_mut<T>(slice: &mut [T]) -> &mut [u8] {
    let size = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), slice.len() * size) }
}
