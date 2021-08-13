use crate::bind;

pub enum AudioDeviceEvent {
    Added {
        timestamp: u32,
        device_id: u32,
        is_microphone: bool,
    },
    Removed {
        timestamp: u32,
        device_id: u32,
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
