use std::ffi::CStr;

use crate::bind;

pub enum DropEvent {
    File {
        timestamp: u32,
        file: String,
        window_id: u32,
    },
    Text {
        timestamp: u32,
        text: String,
        window_id: u32,
    },
    Begin {
        timestamp: u32,
        window_id: u32,
    },
    Complete {
        timestamp: u32,
        window_id: u32,
    },
}

impl From<bind::SDL_DropEvent> for DropEvent {
    fn from(raw: bind::SDL_DropEvent) -> Self {
        match raw.type_ {
            bind::SDL_EventType_SDL_DROPFILE => Self::File {
                timestamp: raw.timestamp,
                file: unsafe { CStr::from_ptr(raw.file) }.to_string_lossy().into(),
                window_id: raw.windowID,
            },
            bind::SDL_EventType_SDL_DROPTEXT => Self::Text {
                timestamp: raw.timestamp,
                text: unsafe { CStr::from_ptr(raw.file) }.to_string_lossy().into(),
                window_id: raw.windowID,
            },
            bind::SDL_EventType_SDL_DROPBEGIN => Self::Begin {
                timestamp: raw.timestamp,
                window_id: raw.windowID,
            },
            bind::SDL_EventType_SDL_DROPCOMPLETE => Self::Complete {
                timestamp: raw.timestamp,
                window_id: raw.windowID,
            },
            _ => unreachable!(),
        }
    }
}
