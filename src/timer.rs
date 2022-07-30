use std::{ffi::c_void, num::NonZeroU32, ops};

use crate::{bind, Result, Sdl, SdlError};

mod ticks;

pub use ticks::*;

/// A callback for [`Timer`], that returns an interval for next calling.
pub trait TimerCallback<'callback>: FnMut() -> u32 + 'callback {}

/// A timer invokes a [`TimerCallback`] with the interval.
pub struct Timer<T> {
    id: NonZeroU32,
    callback: T,
}

impl<T> std::fmt::Debug for Timer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timer")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<'callback, T: TimerCallback<'callback>> Timer<T> {
    /// Constructs a timer with initial interval and callback.
    pub fn new(interval: u32, mut callback: T) -> Result<Self> {
        let data = &mut callback as *mut T;
        let id =
            unsafe { bind::SDL_AddTimer(interval, Some(timer_wrap_handler::<T>), data.cast()) };
        if id == 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                id: unsafe { NonZeroU32::new_unchecked(id as u32) },
                callback,
            })
        }
    }
}

extern "C" fn timer_wrap_handler<'callback, T: TimerCallback<'callback>>(
    _: u32,
    param: *mut c_void,
) -> u32 {
    let callback = unsafe { &mut *(param as *mut T) };
    callback()
}

impl<T> Drop for Timer<T> {
    fn drop(&mut self) {
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
    #[must_use]
    pub fn counter() -> u64 {
        unsafe { bind::SDL_GetPerformanceCounter() }
    }

    /// Returns the numbers of counts per one seconds of the high resolution counter.
    #[must_use]
    pub fn frequency() -> u64 {
        unsafe { bind::SDL_GetPerformanceFrequency() }
    }
}
