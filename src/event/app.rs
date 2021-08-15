//! Events occurring on the application.

use crate::bind;

/// An event on quitting the application.
#[derive(Debug, Clone)]
pub struct QuitEvent {
    /// When this event occurred.
    pub timestamp: u32,
}

impl From<bind::SDL_QuitEvent> for QuitEvent {
    fn from(bind::SDL_QuitEvent { timestamp, .. }: bind::SDL_QuitEvent) -> Self {
        Self { timestamp }
    }
}
