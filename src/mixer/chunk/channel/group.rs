//! Channel groups, reserving [`Channel`] to reuse and control in bulk.

use std::marker::PhantomData;

use super::Channel;
use crate::{bind, mixer::device::MixDevice};

/// A channel group, reserving some [`Channel`]s.
pub struct ChannelGroup<'device> {
    group_id: i32,
    len: usize,
    _phantom: PhantomData<&'device MixDevice<'device>>,
}

impl<'device> ChannelGroup<'device> {
    /// Constructs the new channel group.
    pub fn new(_device: &'device MixDevice<'device>, len: usize) -> Self {
        assert!(1 <= len);
        let prev_len = unsafe { bind::Mix_AllocateChannels(-1) };
        let new_len = prev_len
            + <usize as TryInto<std::os::raw::c_int>>::try_into(len)
                .expect("channel length overflow");
        let allocated = unsafe { bind::Mix_AllocateChannels(new_len) };
        let _ = unsafe { bind::Mix_GroupChannels(prev_len, allocated - 1, prev_len) };
        Self {
            group_id: prev_len,
            len,
            _phantom: PhantomData,
        }
    }

    /// Partitions the group into two groups. The length of first element of tuple will be `left_len`, and the other will be `self.len - left_len`. And halts all the playing channel before doing it.
    ///
    /// # Panics
    ///
    /// Panics if `self.len < left_len`.
    pub fn partition(self, left_len: usize) -> (Self, Self) {
        assert!(left_len <= self.len);
        self.halt_all();
        (
            Self {
                group_id: self.group_id,
                len: left_len,
                _phantom: PhantomData,
            },
            Self {
                group_id: left_len as _,
                len: self.len - left_len,
                _phantom: PhantomData,
            },
        )
    }

    /// Returns the first free mixing channel if exists.
    #[allow(clippy::unnecessary_cast)]
    pub fn first_free(&self) -> Option<Channel> {
        let channel = unsafe { bind::Mix_GroupAvailable(self.group_id) as i32 };
        (0 <= channel).then(|| Channel(channel, PhantomData))
    }

    /// Returns the oldest playing channel in the group.
    pub fn oldest_playing(&self) -> Option<Channel> {
        let oldest = unsafe { bind::Mix_GroupOldest(self.group_id) as _ };
        (0 <= oldest).then(|| Channel(oldest, PhantomData))
    }

    /// Returns the newest playing channel in the group.
    pub fn newest_playing(&self) -> Option<Channel> {
        let newest = unsafe { bind::Mix_GroupNewer(self.group_id) as _ };
        (0 <= newest).then(|| Channel(newest, PhantomData))
    }

    /// Halts all the playing channel.
    pub fn halt_all(&self) {
        let _ = unsafe { bind::Mix_HaltGroup(self.group_id) };
    }

    /// Fade out all the playing channel in milliseconds. And returns the numbers of channels that is fading out now.
    pub fn fade_out_all(&self, fade_out: u32) -> usize {
        unsafe { bind::Mix_FadeOutChannel(self.group_id, fade_out as _) as _ }
    }

    /// Returns the numbers of channels in the group.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the group is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
