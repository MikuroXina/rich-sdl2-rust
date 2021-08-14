use std::{ffi::c_void, num::NonZeroU32, ops};

use crate::{bind, Result, Sdl, SdlError};

mod ticks;

pub use ticks::*;

type TimerCallback<'callback> = Box<dyn FnMut() -> u32 + 'callback>;

pub struct Timer<'callback> {
    id: NonZeroU32,
    raw_callback: *mut TimerCallback<'callback>,
}

impl<'callback> Timer<'callback> {
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

pub fn delay(ms: u32) {
    unsafe { bind::SDL_Delay(ms) }
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
