use std::{ffi::c_void, num::NonZeroU32, ops};

use crate::{bind, Result, Sdl, SdlError};

mod ticks;

pub use ticks::*;

/// A callback for [`Timer`], that returns an interval for next calling.
pub type TimerCallback<'callback> = Box<dyn FnMut() -> u32 + 'callback>;

/// A timer invokes a [`TimerCallback`] with the interval.
pub struct Timer<'callback> {
    id: NonZeroU32,
    raw_callback: *mut TimerCallback<'callback>,
}

impl std::fmt::Debug for Timer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timer").field("id", &self.id).finish()
    }
}

impl<'callback> Timer<'callback> {
    /// Constructs a timer with initial interval and callback.
    pub fn new(interval: u32, callback: TimerCallback<'callback>) -> Result<Self> {
        let wrapped = Box::into_raw(Box::new(callback));
        let id = unsafe { bind::SDL_AddTimer(interval, Some(timer_wrap_handler), wrapped.cast()) };
        if id == 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                id: unsafe { NonZeroU32::new_unchecked(id as u32) },
                raw_callback: wrapped,
            })
        }
    }
}

extern "C" fn timer_wrap_handler(_: u32, param: *mut c_void) -> u32 {
    let callback = unsafe { &mut *(param as *mut TimerCallback) };
    callback()
}

impl Drop for Timer<'_> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.raw_callback) };
        let _ = unsafe { bind::SDL_RemoveTimer(self.id.get() as bind::SDL_TimerID) };
    }
}

/// Stops the current thread for `ms` milliseconds, then returns.
pub fn delay(ms: u32) {
    unsafe { bind::SDL_Delay(ms) }
}

/// A counter for performance analysis.
pub mod performance {
    use crate::bind;

    /// Returns current counts of the high resolution counter.
    pub fn counter() -> u64 {
        unsafe { bind::SDL_GetPerformanceCounter() }
    }

    /// Returns the numbers of counts per one seconds of the high resolution counter.
    pub fn frequency() -> u64 {
        unsafe { bind::SDL_GetPerformanceFrequency() }
    }
}
