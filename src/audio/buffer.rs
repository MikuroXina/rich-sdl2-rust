use std::{mem::MaybeUninit, os::raw::c_int};

use super::format::AudioFormat;
use crate::{
    bind::{self, SDL_MixAudioFormat},
    Result, Sdl, SdlError,
};

#[derive(Debug, Clone)]
pub struct AudioBuffer<T> {
    format: AudioFormat,
    samples: u32,
    channels: u8,
    buffer: Vec<T>,
}

impl<T> AudioBuffer<T> {
    pub fn new(format: AudioFormat, samples: u32, channels: u8, buffer: Vec<T>) -> Self {
        assert_eq!(format.bit_size as usize, std::mem::size_of::<T>() * 8);
        Self {
            format,
            samples,
            channels,
            buffer,
        }
    }

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
        let ret = unsafe { bind::SDL_ConvertAudio(&mut cvt as *mut _) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }
}

impl<T: Default + Clone> AudioBuffer<T> {
    pub fn mix(&self, volume: u8) -> Self {
        let mut dst = self.clone();
        self.mix_in(&mut dst, volume);
        dst
    }

    pub fn mix_in(&self, dst: &mut Self, volume: u8) {
        assert_eq!(self.format.as_raw(), dst.format.as_raw());
        dst.buffer.clear();
        dst.buffer.resize(self.buffer.len(), T::default());
        let len = self.buffer.len() * std::mem::size_of::<T>();
        unsafe {
            bind::SDL_MixAudioFormat(
                as_u8_slice_mut(&mut dst.buffer).as_mut_ptr() as *mut _,
                as_u8_slice(&self.buffer).as_ptr() as *const _,
                self.format.as_raw(),
                len as u32,
                volume.min(bind::SDL_MIX_MAXVOLUME as u8) as c_int,
            )
        }
    }
}

fn as_u8_slice<T>(slice: &[T]) -> &[u8] {
    let size = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const _, slice.len() * size) }
}

fn as_u8_slice_mut<T>(slice: &mut [T]) -> &mut [u8] {
    let size = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut _, slice.len() * size) }
}
