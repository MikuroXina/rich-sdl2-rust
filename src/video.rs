use static_assertions::assert_not_impl_all;
use std::ffi::CStr;
use std::marker::PhantomData;

use crate::{bind, Sdl};

use self::display::Display;

pub mod clipboard;
pub mod color;
pub mod display;
pub mod gamma_ramp;
pub mod geo;
pub mod gl_context;
pub mod renderer;
pub mod screen_saver;
pub mod surface;
pub mod texture;
pub mod vulkan;
pub mod window;

pub struct Video<'sdl> {
    _phantom: PhantomData<&'sdl Sdl>,
}

assert_not_impl_all!(Video: Send, Sync);

impl<'sdl> Video<'sdl> {
    pub fn new(_: &'sdl Sdl) -> Self {
        let ret = unsafe { bind::SDL_InitSubSystem(bind::SDL_INIT_VIDEO) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl video")
        }
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn displays(&self) -> Vec<Display> {
        let ret = unsafe { bind::SDL_GetNumVideoDisplays() };
        if ret <= 0 {
            return vec![];
        }
        (0..ret).map(|idx| Display::new(idx, self)).collect()
    }

    pub fn video_drivers(&self) -> Vec<&str> {
        let num_drivers = unsafe { bind::SDL_GetNumVideoDrivers() };
        if num_drivers <= 0 {
            Sdl::error_then_panic("Getting number of drivers");
        }
        (0..num_drivers)
            .map(|idx| {
                let raw_str = unsafe { bind::SDL_GetVideoDriver(idx) };
                unsafe { CStr::from_ptr(raw_str) }
                    .to_str()
                    .unwrap_or_default()
            })
            .collect()
    }

    pub fn current_driver(&self) -> &str {
        let raw_str = unsafe { bind::SDL_GetCurrentVideoDriver() };
        unsafe { CStr::from_ptr(raw_str) }
            .to_str()
            .unwrap_or_default()
    }

    pub fn has_screen_keyboard(&self) -> bool {
        unsafe { bind::SDL_HasScreenKeyboardSupport() != 0 }
    }
}

impl<'sdl> Drop for Video<'sdl> {
    fn drop(&mut self) {
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_VIDEO) }
    }
}
