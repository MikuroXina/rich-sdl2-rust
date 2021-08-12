use crate::{bind, Sdl};

pub fn delay(ms: u32) {
    unsafe { bind::SDL_Delay(ms) }
}
