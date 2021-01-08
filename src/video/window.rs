use bitflags::bitflags;
use std::ptr::NonNull;

use super::{display::Display, pixel_format::kind::PixelFormatKind};
use crate::{bind, Result, Sdl, Video};

mod border;
mod brightness;
mod builder;
mod config;
mod grab;
mod hit_test;
pub mod message_box;
mod state;

pub use border::*;
pub use brightness::*;
pub use builder::{WindowBuilder, WindowPos};
pub use config::*;
pub use grab::*;
pub use hit_test::*;
pub use state::*;

bitflags! {
    struct WindowFlags: u32 {
        const FULLSCREEN = bind::SDL_WindowFlags_SDL_WINDOW_FULLSCREEN;
        const FULLSCREEN_DESKTOP = bind::SDL_WindowFlags_SDL_WINDOW_FULLSCREEN_DESKTOP;
        const OPENGL = bind::SDL_WindowFlags_SDL_WINDOW_OPENGL;
        const VULKAN = bind::SDL_WindowFlags_SDL_WINDOW_VULKAN;
        const METAL = bind::SDL_WindowFlags_SDL_WINDOW_METAL;
        const SHOWN = bind::SDL_WindowFlags_SDL_WINDOW_SHOWN;
        const HIDDEN = bind::SDL_WindowFlags_SDL_WINDOW_HIDDEN;
        const BORDERLESS = bind::SDL_WindowFlags_SDL_WINDOW_BORDERLESS;
        const RESIZABLE = bind::SDL_WindowFlags_SDL_WINDOW_RESIZABLE;
        const MINIMIZED = bind::SDL_WindowFlags_SDL_WINDOW_MINIMIZED;
        const MAXIMIZED = bind::SDL_WindowFlags_SDL_WINDOW_MAXIMIZED;
        const INPUT_GRABBED = bind::SDL_WindowFlags_SDL_WINDOW_INPUT_GRABBED;
        const INPUT_FOCUS = bind::SDL_WindowFlags_SDL_WINDOW_INPUT_FOCUS;
        const MOUSE_FOCUS = bind::SDL_WindowFlags_SDL_WINDOW_MOUSE_FOCUS;
        const FOREIGN = bind::SDL_WindowFlags_SDL_WINDOW_FOREIGN;
        const ALLOW_HIGHDPI = bind::SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI;
        const MOUSE_CAPTURE = bind::SDL_WindowFlags_SDL_WINDOW_MOUSE_CAPTURE;
    }
}

pub struct Window<'video> {
    window: NonNull<bind::SDL_Window>,
    video: &'video Video<'video>,
}

impl<'video> Window<'video> {
    pub fn from_id(id: u32, video: &'video Video) -> Option<Self> {
        let raw = unsafe { bind::SDL_GetWindowFromID(id) };
        NonNull::new(raw).map(|window| Self { window, video })
    }

    pub fn grabbed(video: &'video Video) -> Option<Self> {
        let raw = unsafe { bind::SDL_GetGrabbedWindow() };
        NonNull::new(raw).map(|window| Self { window, video })
    }

    pub fn as_ptr(&self) -> *mut bind::SDL_Window {
        self.window.as_ptr()
    }

    pub fn state(&self) -> WindowState {
        let flag_bits = unsafe { bind::SDL_GetWindowFlags(self.as_ptr()) };
        WindowFlags::from_bits_truncate(flag_bits).into()
    }

    pub fn display(&self) -> Option<Display> {
        let ret = unsafe { bind::SDL_GetWindowDisplayIndex(self.as_ptr()) };
        if ret < 0 {
            None
        } else {
            Some(Display::new(ret, &self.video))
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { bind::SDL_GetWindowID(self.as_ptr()) }
    }

    pub fn pixel_format(&self) -> PixelFormatKind {
        unsafe { bind::SDL_GetWindowPixelFormat(self.as_ptr()) }.into()
    }

    pub fn show(&self) {
        unsafe { bind::SDL_ShowWindow(self.as_ptr()) }
    }

    pub fn hide(&self) {
        unsafe { bind::SDL_HideWindow(self.as_ptr()) }
    }

    pub fn raise(&self) {
        unsafe { bind::SDL_RaiseWindow(self.as_ptr()) }
    }

    pub fn full_screen(&self) -> Result<()> {
        let ret = unsafe {
            bind::SDL_SetWindowFullscreen(
                self.as_ptr(),
                bind::SDL_WindowFlags_SDL_WINDOW_FULLSCREEN,
            )
        };
        if ret != 0 {
            return Err(crate::SdlError::Others { msg: Sdl::error() });
        }
        Ok(())
    }

    pub fn maximize(&self) {
        unsafe { bind::SDL_MaximizeWindow(self.as_ptr()) }
    }

    pub fn minimize(&self) {
        unsafe { bind::SDL_MinimizeWindow(self.as_ptr()) }
    }

    pub fn restore(&self) {
        unsafe { bind::SDL_RestoreWindow(self.as_ptr()) }
    }

    // TODO(MikuroXina): surface
    // TODO(MikuroXina): set icon
    // TODO(MikuroXina): open gl context
}

impl<'video> Drop for Window<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyWindow(self.window.as_ptr()) }
    }
}
