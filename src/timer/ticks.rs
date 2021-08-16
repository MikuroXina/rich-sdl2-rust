use std::ops;

use crate::{Sdl, bind};

/// An elapsed time from when SDL2 has initialized. Please note that the value formed 32-bit, overflowing after about 49 days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ticks(pub u32);

impl Ticks {
    /// Gets a current [`Ticks`].
    pub fn now(_: &Sdl) -> Self {
        let ticks = unsafe { bind::SDL_GetTicks() };
        Ticks(ticks)
    }
}

impl ops::Add<u32> for Ticks {
    type Output = Ticks;

    fn add(self, rhs: u32) -> Self::Output {
        Ticks(self.0 + rhs)
    }
}

impl ops::Add<Ticks> for u32 {
    type Output = Ticks;

    fn add(self, rhs: Ticks) -> Self::Output {
        Ticks(self + rhs.0)
    }
}

impl ops::Add<Ticks> for Ticks {
    type Output = Ticks;

    fn add(self, rhs: Ticks) -> Self::Output {
        Ticks(self.0 + rhs.0)
    }
}

impl ops::Sub for Ticks {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as i32 - rhs.0 as i32
    }
}
