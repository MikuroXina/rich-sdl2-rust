use bitflags::bitflags;
use std::ptr::NonNull;

use crate::{bind, Video};

mod builder;

pub use builder::{WindowBuilder, WindowPos};

use super::display::Display;

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

#[derive(Debug)]
pub enum WindowFormat {
    Normal,
    FullScreen,
    FullScreenWithCurrentDesktop,
    Minimized,
    Maximized,
}

impl From<WindowFlags> for WindowFormat {
    fn from(flags: WindowFlags) -> Self {
        use WindowFormat::*;
        if flags.contains(WindowFlags::FULLSCREEN) {
            FullScreen
        } else if flags.contains(WindowFlags::FULLSCREEN_DESKTOP) {
            FullScreenWithCurrentDesktop
        } else if flags.contains(WindowFlags::MINIMIZED) {
            Minimized
        } else if flags.contains(WindowFlags::MAXIMIZED) {
            Maximized
        } else {
            Normal
        }
    }
}

#[derive(Debug)]
pub enum WindowContextKind {
    Software,
    OpenGl,
    Vulkan,
    Metal,
}

impl From<WindowFlags> for WindowContextKind {
    fn from(flags: WindowFlags) -> Self {
        use WindowContextKind::*;
        if flags.contains(WindowFlags::OPENGL) {
            OpenGl
        } else if flags.contains(WindowFlags::VULKAN) {
            Vulkan
        } else if flags.contains(WindowFlags::METAL) {
            Metal
        } else {
            Software
        }
    }
}

pub struct WindowState {
    pub format: WindowFormat,
    pub context_kind: WindowContextKind,
    pub hidden: bool,
    pub borderless: bool,
    pub resizable: bool,
    pub input_grabbed: bool,
    pub on_focus: bool,
    pub on_mouse: bool,
    pub foreign: bool,
    pub allow_high_dpi: bool,
    pub mouse_capture: bool,
}

impl From<WindowFlags> for WindowState {
    fn from(flags: WindowFlags) -> Self {
        Self {
            format: flags.into(),
            context_kind: flags.into(),
            hidden: flags.contains(WindowFlags::HIDDEN),
            borderless: flags.contains(WindowFlags::BORDERLESS),
            resizable: flags.contains(WindowFlags::RESIZABLE),
            input_grabbed: flags.contains(WindowFlags::INPUT_GRABBED),
            on_focus: flags.contains(WindowFlags::INPUT_FOCUS),
            on_mouse: flags.contains(WindowFlags::MOUSE_FOCUS),
            foreign: flags.contains(WindowFlags::FOREIGN),
            allow_high_dpi: flags.contains(WindowFlags::ALLOW_HIGHDPI),
            mouse_capture: flags.contains(WindowFlags::MOUSE_CAPTURE),
        }
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

    // TODO(MikuroXina): window brightness and gamma
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
