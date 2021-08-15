//! An event related on an audio device.

use crate::bind;

/// An event occurs on an audio device was added/removed.
#[derive(Debug, Clone)]
pub enum AudioDeviceEvent {
    /// An event when an audio device was added.
    Added {
        /// When this event occurred.
        timestamp: u32,
        /// The id of an added audio device.
        device_id: u32,
        /// Whether it is an audio device to record.
        is_microphone: bool,
    },
    /// An event when an audio device was removed.
    Removed {
        /// When this event occurred.
        timestamp: u32,
        /// The id of a removed audio device.
        device_id: u32,
        /// Whether it is an audio device to record.
        is_microphone: bool,
    },
}

impl From<bind::SDL_AudioDeviceEvent> for AudioDeviceEvent {
    fn from(raw: bind::SDL_AudioDeviceEvent) -> Self {
        match raw.type_ {
            bind::SDL_EventType_SDL_AUDIODEVICEADDED => AudioDeviceEvent::Added {
                timestamp: raw.timestamp,
                device_id: raw.which,
                is_microphone: raw.iscapture != 0,
            },
            bind::SDL_EventType_SDL_AUDIODEVICEREMOVED => AudioDeviceEvent::Removed {
                timestamp: raw.timestamp,
                device_id: raw.which,
                is_microphone: raw.iscapture != 0,
            },
            _ => unreachable!(),
        }
    }
}
