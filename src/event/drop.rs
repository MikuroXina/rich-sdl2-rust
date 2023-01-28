//! Events on to drag and drop something to a window.

use std::ffi::CStr;

use crate::{bind, EnumInt};

/// An event occurred when the item was dropped on a window.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DropEvent {
    /// A file was dropped.
    File {
        /// When this event occurred.
        timestamp: u32,
        /// A path of the file dropped.
        file: String,
        /// An id of the window that was dropped.
        window_id: u32,
    },
    /// A text was dropped.
    Text {
        /// When this event occurred.
        timestamp: u32,
        /// A dropped text.
        text: String,
        /// An id of the window that was dropped.
        window_id: u32,
    },
    /// A dragging began.
    Begin {
        /// When this event occurred.
        timestamp: u32,
        /// An id of the window that was started to drag.
        window_id: u32,
    },
    /// The dragging completed.
    Complete {
        /// When this event occurred.
        timestamp: u32,
        /// An id of the window that was ended to drag.
        window_id: u32,
    },
}

impl From<bind::SDL_DropEvent> for DropEvent {
    fn from(raw: bind::SDL_DropEvent) -> Self {
        match raw.type_ as EnumInt {
            bind::SDL_DROPFILE => Self::File {
                timestamp: raw.timestamp,
                file: unsafe { CStr::from_ptr(raw.file) }.to_string_lossy().into(),
                window_id: raw.windowID,
            },
            bind::SDL_DROPTEXT => Self::Text {
                timestamp: raw.timestamp,
                text: unsafe { CStr::from_ptr(raw.file) }.to_string_lossy().into(),
                window_id: raw.windowID,
            },
            bind::SDL_DROPBEGIN => Self::Begin {
                timestamp: raw.timestamp,
                window_id: raw.windowID,
            },
            bind::SDL_DROPCOMPLETE => Self::Complete {
                timestamp: raw.timestamp,
                window_id: raw.windowID,
            },
            _ => unreachable!(),
        }
    }
}
