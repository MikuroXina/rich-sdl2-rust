use static_assertions::assert_not_impl_all;
use std::{ffi::CStr, ptr::NonNull};

use crate::bind;

pub mod axis;
pub mod button;
pub mod map;

pub struct GameController {
    pub(in crate::event) ptr: NonNull<bind::SDL_GameController>,
}

assert_not_impl_all!(GameController: Send, Sync);

impl GameController {
    pub fn mapping(&self) -> String {
        let ptr = unsafe { bind::SDL_GameControllerMapping(self.ptr.as_ptr()) };
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let ret = cstr.to_string_lossy().to_string();
        unsafe { bind::SDL_free(ptr as *mut _) };
        ret
    }
}

pub struct GameControllerSet {
    controls: Vec<GameController>,
}

impl GameControllerSet {
    pub fn new() -> Self {
        let num_controls = unsafe {
            bind::SDL_InitSubSystem(bind::SDL_INIT_JOYSTICK);
            bind::SDL_NumJoysticks()
        };
        let controls = (0..num_controls)
            .filter(|&index| unsafe { bind::SDL_IsGameController(index) != 0 })
            .filter_map(|index| {
                let raw = unsafe { bind::SDL_GameControllerOpen(index) };
                NonNull::new(raw)
            })
            .map(|ptr| GameController { ptr })
            .collect();
        Self { controls }
    }

    pub fn controllers(&self) -> &[GameController] {
        &self.controls
    }
}

impl Drop for GameControllerSet {
    fn drop(&mut self) {
        for control in &mut self.controls {
            unsafe { bind::SDL_GameControllerClose(control.ptr.as_ptr()) }
        }
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_JOYSTICK) }
    }
}
