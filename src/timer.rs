use std::ops;

use crate::{bind, Sdl};

pub fn delay(ms: u32) {
    unsafe { bind::SDL_Delay(ms) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ticks(pub u32);

impl Ticks {
    pub fn now() -> Self {
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

pub mod performance {
    use crate::bind;

    pub fn counter() -> u64 {
        unsafe { bind::SDL_GetPerformanceCounter() }
    }

    pub fn frequency() -> u64 {
        unsafe { bind::SDL_GetPerformanceFrequency() }
    }
}
