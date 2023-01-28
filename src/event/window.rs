//! Events for the window.

use crate::{
    bind,
    geo::{Point, Size},
    EnumInt,
};

/// The details what occurred in [`WindowEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
    /// The window given focus. When this event was occurred, you should call [`crate::video::window::Window::set_input_focus`] on the window.
    TakeFocus,
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
        Self {
            timestamp,
            window_id,
            details: match event as EnumInt {
                bind::SDL_WINDOWEVENT_SHOWN => WindowEventDetails::Shown,
                bind::SDL_WINDOWEVENT_HIDDEN => WindowEventDetails::Hidden,
                bind::SDL_WINDOWEVENT_EXPOSED => WindowEventDetails::Exposed,
                bind::SDL_WINDOWEVENT_MOVED => {
                    WindowEventDetails::Moved(Point { x: data1, y: data2 })
                }
                bind::SDL_WINDOWEVENT_RESIZED => WindowEventDetails::Resized(Size {
                    width: data1 as u32,
                    height: data2 as u32,
                }),
                bind::SDL_WINDOWEVENT_SIZE_CHANGED => WindowEventDetails::SizeChanged(Size {
                    width: data1 as u32,
                    height: data2 as u32,
                }),
                bind::SDL_WINDOWEVENT_MINIMIZED => WindowEventDetails::Minimized,
                bind::SDL_WINDOWEVENT_MAXIMIZED => WindowEventDetails::Maximized,
                bind::SDL_WINDOWEVENT_RESTORED => WindowEventDetails::Restored,
                bind::SDL_WINDOWEVENT_ENTER => WindowEventDetails::Enter,
                bind::SDL_WINDOWEVENT_LEAVE => WindowEventDetails::Leave,
                bind::SDL_WINDOWEVENT_FOCUS_GAINED => WindowEventDetails::FocusGained,
                bind::SDL_WINDOWEVENT_FOCUS_LOST => WindowEventDetails::FocusLost,
                bind::SDL_WINDOWEVENT_CLOSE => WindowEventDetails::Close,
                bind::SDL_WINDOWEVENT_TAKE_FOCUS => WindowEventDetails::TakeFocus,
                other => todo!("{other} is not implemented"),
            },
        }
    }
}
