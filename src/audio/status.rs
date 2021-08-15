//! A status of an audio device.

use crate::bind;

/// A playing status of an audio device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioStatus {
    /// An audio device is closed or on error.
    Stopped,
    /// An audio device is open and playing the sound.
    Playing,
    /// An audio device is open but not playing the sound.
    Paused,
}

impl From<bind::SDL_AudioStatus> for AudioStatus {
    fn from(raw: bind::SDL_AudioStatus) -> Self {
        match raw {
            bind::SDL_AudioStatus_SDL_AUDIO_STOPPED => Self::Stopped,
            bind::SDL_AudioStatus_SDL_AUDIO_PLAYING => Self::Playing,
            bind::SDL_AudioStatus_SDL_AUDIO_PAUSED => Self::Paused,
            _ => unreachable!(),
        }
    }
}
