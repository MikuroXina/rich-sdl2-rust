use static_assertions::assert_not_impl_all;
use std::ptr::NonNull;

use super::{color::pixel::kind::PixelFormatKind, display::Display};
use crate::surface::window::WindowSurface;
use crate::surface::Surface;
use crate::{bind, Result, Sdl, SdlError, Video};

mod border;
mod brightness;
mod builder;
mod config;
pub mod cursor;
mod grab;
mod hit_test;
pub mod message_box;
mod state;

pub use border::*;
pub use brightness::*;
use builder::WindowFlags;
pub use builder::{WindowBuilder, WindowCoord};
pub use config::*;
pub use grab::*;
pub use hit_test::*;
pub use state::*;

pub struct Window<'video> {
    window: NonNull<bind::SDL_Window>,
    video: &'video Video<'video>,
}

impl std::fmt::Debug for Window<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window").field("id", &self.id()).finish()
    }
}

assert_not_impl_all!(Window: Send, Sync);

impl<'video> Window<'video> {
    pub fn from_id(id: u32, video: &'video Video) -> Option<Self> {
        let raw = unsafe { bind::SDL_GetWindowFromID(id) };
        NonNull::new(raw).map(|window| Self { window, video })
    }

    pub fn grabbed(video: &'video Video) -> Option<Self> {
        let raw = unsafe { bind::SDL_GetGrabbedWindow() };
        NonNull::new(raw).map(|window| Self { window, video })
    }

    pub fn mouse_focused(video: &'video Video) -> Option<Self> {
        let raw = unsafe { bind::SDL_GetMouseFocus() };
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
        (0 <= ret).then(|| Display::new(ret, self.video))
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

    pub fn set_icon(&self, icon: &impl Surface) {
        unsafe { bind::SDL_SetWindowIcon(self.as_ptr(), icon.as_ptr().as_ptr()) }
    }

    pub fn is_screen_keyboard_shown(&self) -> bool {
        unsafe { bind::SDL_IsScreenKeyboardShown(self.as_ptr()) != 0 }
    }

    pub fn surface(&self) -> WindowSurface {
        WindowSurface::new(self)
    }
}

impl<'video> Drop for Window<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyWindow(self.window.as_ptr()) }
    }
}
