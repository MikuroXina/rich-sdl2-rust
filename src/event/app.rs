use crate::bind;

#[derive(Debug, Clone)]
pub struct QuitEvent {
    pub timestamp: u32,
}

impl From<bind::SDL_QuitEvent> for QuitEvent {
    fn from(bind::SDL_QuitEvent { timestamp, .. }: bind::SDL_QuitEvent) -> Self {
        Self { timestamp }
    }
}
