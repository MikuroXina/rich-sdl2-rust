//! Channels to play a [`MixChunk`].

use std::{marker::PhantomData, os::raw::c_int, ptr::NonNull};

use super::MixChunk;
use crate::{
    mixer::{bind, device::MixDevice},
    Result, Sdl, SdlError,
};

pub use group::*;
pub use pause::*;

pub mod effect;
mod group;
mod pause;

/// Loops on playing in [`PlayOptions`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlayLoops {
    /// Playing infinitely.
    Infinite,
    /// Playing only once.
    OneShot,
    /// Playing the specified number of times.
    Times(u32),
}

impl PlayLoops {
    fn into_raw(self) -> c_int {
        match self {
            PlayLoops::Infinite => -1,
            PlayLoops::OneShot => 0,
            PlayLoops::Times(n) => n.saturating_sub(1) as _,
        }
    }
}

impl Default for PlayLoops {
    fn default() -> Self {
        Self::OneShot
    }
}

/// Options to play a chunk by [`Channel::play`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlayOptions {
    /// Loops on playing.
    pub loops: PlayLoops,
    /// The maximum duration of playing in milliseconds. Stopping by `loops` is prior to by `duration`.
    pub duration: Option<u32>,
}

/// A mixing channel for playing a [`MixChunk`].
#[derive(Debug, PartialEq, Eq)]
pub struct Channel<'device>(i32, PhantomData<&'device MixDevice<'device>>);

impl<'device> Channel<'device> {
    /// Returns the output volume of the channel. The volume is in `0..=128`.
    pub fn volume(&self) -> u32 {
        unsafe { bind::Mix_Volume(self.0, -1) as _ }
    }

    /// Sets the output volume of the channel. The volume is clamped to `0..=128`.
    pub fn set_volume(&self, volume: u32) {
        let _ = unsafe { bind::Mix_Volume(self.0, volume.clamp(0, 128) as _) };
    }

    /// Starts to play a chunk.
    pub fn play(self, chunk: &MixChunk, options: PlayOptions) -> Result<Self> {
        let id = unsafe {
            bind::Mix_PlayChannelTimed(
                self.0,
                chunk.ptr.as_ptr(),
                options.loops.into_raw(),
                options.duration.map_or(-1, |d| d as _),
            )
        };
        if id == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self(id, PhantomData))
        }
    }

    /// Starts to play a chunk with fade-in time in milliseconds.
    pub fn play_fade_in(
        self,
        chunk: &MixChunk,
        fade_in: u32,
        options: PlayOptions,
    ) -> Result<Self> {
        let id = unsafe {
            bind::Mix_FadeInChannelTimed(
                self.0,
                chunk.ptr.as_ptr(),
                options.loops.into_raw(),
                fade_in as _,
                options.duration.map_or(-1, |d| d as _),
            )
        };
        if id == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self(id, PhantomData))
        }
    }

    /// Pauses playing and returns the [`Pauser`], or `None` if it is free.
    pub fn pause(&'device mut self) -> Option<Pauser<'device>> {
        Pauser::pause(self)
    }

    /// Halts playing on the channel.
    pub fn halt(&self) {
        let _ = unsafe { bind::Mix_HaltChannel(self.0) };
    }

    /// Fade out playing on the channel in milliseconds. And returns the numbers of channels that is fading out.
    pub fn fade_out(&self, fade_out: u32) -> usize {
        unsafe { bind::Mix_FadeOutChannel(self.0, fade_out as _) as _ }
    }

    /// Returns whether the channel is playing.
    pub fn is_playing(&self) -> bool {
        unsafe { bind::Mix_Playing(self.0) != 0 }
    }

    /// Returns the playing chunk if exists.
    pub fn playing_chunk(&self) -> Option<MixChunk> {
        if self.0 == -1 {
            return None;
        }
        let ptr = unsafe { bind::Mix_GetChunk(self.0) };
        (!ptr.is_null()).then(|| MixChunk {
            ptr: NonNull::new(ptr).unwrap(),
            _phantom: PhantomData,
        })
    }
}
