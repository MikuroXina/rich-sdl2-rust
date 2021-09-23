//! Events for the window.

use crate::{
    bind,
    geo::{Point, Size},
    EnumInt,
};

/// The details what occurred in [`WindowEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEventDetails {
    /// The window became to be shown.
    Shown,
    /// The window became to be hidden.
    Hidden,
    /// The window was exposed.
    Exposed,
    /// The windows was moved.
    Moved(Point),
    /// The window was resized by manually.
    Resized(Size),
    /// The window was resized by programmatically.
    SizeChanged(Size),
    /// The window was minimized.
    Minimized,
    /// The window was maximized.
    Maximized,
    /// The window was restored from max/min.
    Restored,
    /// The cursor was entered in the window.
    Enter,
    /// The cursor was left from the window.
    Leave,
    /// The window was focused.
    FocusGained,
    /// The window lost focused.
    FocusLost,
    /// The window was closed.
    Close,
}

/// An event on interacting to the window.
#[derive(Debug, Clone)]
pub struct WindowEvent {
    /// When this event occurred.
    pub timestamp: u32,
    /// The id of the window focused.
    pub window_id: u32,
    /// The kind of the event.
    pub details: WindowEventDetails,
}

impl From<bind::SDL_WindowEvent> for WindowEvent {
    fn from(
        bind::SDL_WindowEvent {
            timestamp,
            windowID: window_id,
            event,
            data1,
            data2,
            ..
        }: bind::SDL_WindowEvent,
    ) -> Self {
        use WindowEventDetails::*;
        Self {
            timestamp,
            window_id,
            details: match event as EnumInt {
                bind::SDL_WINDOWEVENT_SHOWN => Shown,
                bind::SDL_WINDOWEVENT_HIDDEN => Hidden,
                bind::SDL_WINDOWEVENT_EXPOSED => Exposed,
                bind::SDL_WINDOWEVENT_MOVED => Moved(Point { x: data1, y: data2 }),
                bind::SDL_WINDOWEVENT_RESIZED => Resized(Size {
                    width: data1 as u32,
                    height: data2 as u32,
                }),
                bind::SDL_WINDOWEVENT_SIZE_CHANGED => SizeChanged(Size {
                    width: data1 as u32,
                    height: data2 as u32,
                }),
                bind::SDL_WINDOWEVENT_MINIMIZED => Minimized,
                bind::SDL_WINDOWEVENT_MAXIMIZED => Maximized,
                bind::SDL_WINDOWEVENT_RESTORED => Restored,
                bind::SDL_WINDOWEVENT_ENTER => Enter,
                bind::SDL_WINDOWEVENT_LEAVE => Leave,
                bind::SDL_WINDOWEVENT_FOCUS_GAINED => FocusGained,
                bind::SDL_WINDOWEVENT_FOCUS_LOST => FocusLost,
                bind::SDL_WINDOWEVENT_CLOSE => Close,
                _ => todo!(),
            },
        }
    }
}
