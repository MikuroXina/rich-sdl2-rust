//! Pauser for a [`MixMusic`].

use std::marker::PhantomData;

use super::MixMusic;
use crate::bind;

/// A pauser to pause playing audio on the [`Channel`]. Unpause to drop this.
pub struct Pauser<'music>(PhantomData<&'music mut MixMusic<'music>>);

impl<'music> Pauser<'music> {
    pub(super) fn pause(_: &'music mut MixMusic<'music>) -> Self {
        unsafe { bind::Mix_PauseMusic() }
        Self(PhantomData)
    }
}

impl Drop for Pauser<'_> {
    fn drop(&mut self) {
        unsafe { bind::Mix_ResumeMusic() }
    }
}
