use std::{ffi::c_void, marker::PhantomData, num::NonZeroU32, ops, ptr::addr_of_mut};

use crate::{bind, Result, Sdl, SdlError};

mod ticks;

pub use ticks::*;

/// A callback for [`Timer`], that returns an interval for next calling.
pub trait TimerCallback<'callback>: FnMut() -> u32 + 'callback {}

/// A timer invokes a [`TimerCallback`] with the interval.
pub struct Timer<'sdl, T> {
    id: NonZeroU32,
    callback: T,
    _phantom: PhantomData<&'sdl Sdl>,
}

impl<T> std::fmt::Debug for Timer<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timer")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<'sdl, 'callback, T: TimerCallback<'callback>> Timer<'sdl, T> {
    /// Constructs a timer with initial interval and callback.
    ///
    /// The timing may be inaccurate because of OS scheduling. Make sure to check the current time in your callback.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to create a new timer.
    pub fn new(sdl: &'sdl Sdl, interval: u32, mut callback: T) -> Result<Self> {
        let ret = unsafe { bind::SDL_InitSubSystem(bind::SDL_INIT_TIMER) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl timer");
        }

        let data = addr_of_mut!(callback);
        let id =
            unsafe { bind::SDL_AddTimer(interval, Some(timer_wrap_handler::<T>), data.cast()) };
        if id == 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                id: unsafe { NonZeroU32::new_unchecked(id as u32) },
                callback,
                _phantom: PhantomData,
            })
        }
    }
}

extern "C" fn timer_wrap_handler<'callback, T: TimerCallback<'callback>>(
    _: u32,
    param: *mut c_void,
) -> u32 {
    let callback = unsafe { &mut *param.cast::<T>() };
    callback()
}

impl<'sdl, T> Drop for Timer<'sdl, T> {
    fn drop(&mut self) {
        unsafe {
            let _ = bind::SDL_RemoveTimer(self.id.get() as bind::SDL_TimerID);
            bind::SDL_QuitSubSystem(bind::SDL_INIT_TIMER);
        }
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
