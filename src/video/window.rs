use bitflags::bitflags;
use std::ptr::NonNull;

use super::display::Display;
use crate::{bind, Video};

mod brightness;
mod builder;
mod state;

pub use brightness::*;
pub use builder::{WindowBuilder, WindowPos};
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

    // TODO(MikuroXina): grab
    // TODO(MikuroXina): get id and pixel format
    // TODO(MikuroXina): get/set max size, min size, opacity, position, size and title
    // TODO(MikuroXina): set icon, input focus and hit test
    // TODO(MikuroXina): border widths
    // TODO(MikuroXina): add frame/frameless
    // TODO(MikuroXina): modal and message box
    // TODO(MikuroXina): show, hide, raise and restore
    // TODO(MikuroXina): full screen, maximize and minimize
}

impl<'video> Drop for Window<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyWindow(self.window.as_ptr()) }
    }
}
