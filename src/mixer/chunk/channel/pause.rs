use std::marker::PhantomData;

use super::Channel;
use crate::{bind, mixer::device::MixDevice};

/// A pauser to pause playing audio on the [`Channel`]. Unpause to drop this.
pub struct Pauser<'channel>(i32, PhantomData<&'channel mut Channel<'channel>>);

impl<'channel> Pauser<'channel> {
    /// Pauses playing on the [`Channel`], or `None` if it is free.
    pub fn pause(channel: &'channel Channel<'channel>) -> Option<Self> {
        if channel.0 == -1 {
            return None;
        }
        unsafe { bind::Mix_Pause(channel.0) }
        Some(Self(channel.0, PhantomData))
    }

    /// Pauses all the playing channels.
    pub fn pause_all(_device: &'channel MixDevice<'channel>) -> Self {
        unsafe { bind::Mix_Pause(-1) }
        Self(-1, PhantomData)
    }
}

impl Drop for Pauser<'_> {
    fn drop(&mut self) {
        unsafe { bind::Mix_Resume(self.0) }
    }
}
