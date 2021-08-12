use crate::bind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioStatus {
    Stopped,
    Playing,
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
