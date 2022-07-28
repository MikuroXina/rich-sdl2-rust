//! This module provides wrapper for SDL2_mixer and abstractions of audio playing APIs.

pub mod chunk;
pub mod device;
pub mod music;

use crate::{bind, Result, SdlError, SdlVersion};
use bitflags::bitflags;
use static_assertions::assert_not_impl_all;
use std::{cell::Cell, marker::PhantomData};

bitflags! {
    /// A format flag to use on initializing of [`Mix`].
    pub struct FormatFlag: u32 {
        /// Using flac audio format.
        const FLAC = 1 << 0;
        /// Using mod audio format.
        const MOD = 1 << 1;
        /// Using mpeg-1 audio layer-3 format.
        const MP3 = 1 << 2;
        /// Using ogg vorbis audio format.
        const OGG = 1 << 3;
        /// Using midi format.
        const MIDI = 1 << 4;
        /// Using opus audio format.
        const OPUS = 1 << 5;
    }
}

/// A root SDL2_mixer controller.
pub struct Mix {
    _phantom: PhantomData<Cell<u8>>,
}

assert_not_impl_all!(Mix: Send, Sync);

impl Mix {
    /// Constructs a root controller, or `Err` if the format is not supported.
    pub fn new(flag: FormatFlag) -> Result<Self> {
        let ret = unsafe { bind::Mix_Init(flag.bits as _) };
        if !flag.contains(FormatFlag::from_bits_truncate(ret as _)) {
            Err(SdlError::UnsupportedFeature)
        } else {
            Ok(Self {
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the library version of SDL2_mixer.
    pub fn version() -> SdlVersion {
        let raw = unsafe { &*bind::Mix_Linked_Version() };
        SdlVersion {
            major: raw.major,
            minor: raw.minor,
            patch: raw.patch,
        }
    }
}

impl Drop for Mix {
    fn drop(&mut self) {
        unsafe { bind::Mix_Quit() }
    }
}
