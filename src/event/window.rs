use crate::bind;

#[derive(Debug, Clone)]
pub enum WindowEventDetails {
    Shown,
    Hidden,
    Exposed,
    Moved { x: i32, y: i32 },
    Resized { width: u32, height: u32 },
    SizeChanged { width: u32, height: u32 },
    Minimized,
    Maximized,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
}

#[derive(Debug, Clone)]
pub struct WindowEvent {
    pub timestamp: u32,
    pub window_id: u32,
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
            details: match event as u32 {
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_SHOWN => Shown,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_HIDDEN => Hidden,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_EXPOSED => Exposed,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_MOVED => Moved { x: data1, y: data2 },
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_RESIZED => Resized {
                    width: data1 as u32,
                    height: data2 as u32,
                },
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_SIZE_CHANGED => SizeChanged {
                    width: data1 as u32,
                    height: data2 as u32,
                },
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_MINIMIZED => Minimized,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_MAXIMIZED => Maximized,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_RESTORED => Restored,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_ENTER => Enter,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_LEAVE => Leave,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_FOCUS_GAINED => FocusGained,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_FOCUS_LOST => FocusLost,
                bind::SDL_WindowEventID_SDL_WINDOWEVENT_CLOSE => Close,
                _ => todo!(),
            },
        }
    }
}
