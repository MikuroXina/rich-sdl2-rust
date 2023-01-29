//! Window managements, graphics and mouse controls.

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use static_assertions::assert_not_impl_all;
use std::{
    ffi::c_void,
    mem::MaybeUninit,
    ptr::{addr_of_mut, NonNull},
};

use super::{color::pixel::kind::PixelFormatKind, display::Display};
use crate::surface::window::WindowSurface;
use crate::surface::Surface;
use crate::{bind, EnumInt, Result, Sdl, SdlError, Video};

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

/// A window made by SDL2.
pub struct Window<'video> {
    window: NonNull<bind::SDL_Window>,
    video: &'video Video<'video>,
}

impl std::fmt::Debug for Window<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}

assert_not_impl_all!(Window: Send, Sync);

impl<'video> Window<'video> {
    /// Gets a window from the window id, or `None` if does not exist.
    #[must_use]
    pub fn from_id(id: u32, video: &'video Video) -> Option<Self> {
        let raw = unsafe { bind::SDL_GetWindowFromID(id) };
        NonNull::new(raw).map(|window| Self { window, video })
    }

    /// Gets a grabbed window, or `None` if does not exist.
    #[must_use]
    pub fn grabbed(video: &'video Video) -> Option<Self> {
        let raw = unsafe { bind::SDL_GetGrabbedWindow() };
        NonNull::new(raw).map(|window| Self { window, video })
    }

    /// Gets a focused window, or `None` if does not exist.
    #[must_use]
    pub fn mouse_focused(video: &'video Video) -> Option<Self> {
        let raw = unsafe { bind::SDL_GetMouseFocus() };
        NonNull::new(raw).map(|window| Self { window, video })
    }

    pub(crate) fn as_ptr(&self) -> *mut bind::SDL_Window {
        self.window.as_ptr()
    }

    /// Returns the state of the window.
    #[must_use]
    pub fn state(&self) -> WindowState {
        let flag_bits = unsafe { bind::SDL_GetWindowFlags(self.as_ptr()) };
        WindowFlags::from_bits_truncate(flag_bits).into()
    }

    /// Returns the display at the window, or `None` if unavailable.
    #[must_use]
    pub fn display(&self) -> Option<Display> {
        let ret = unsafe { bind::SDL_GetWindowDisplayIndex(self.as_ptr()) };
        (0 <= ret).then(|| Display::new(ret, self.video))
    }

    /// Returns the window id.
    #[must_use]
    pub fn id(&self) -> u32 {
        unsafe { bind::SDL_GetWindowID(self.as_ptr()) }
    }

    /// Returns the pixel format of the window context.
    #[must_use]
    pub fn pixel_format(&self) -> PixelFormatKind {
        PixelFormatKind::from_raw(
            (unsafe { bind::SDL_GetWindowPixelFormat(self.as_ptr()) }) as EnumInt,
        )
    }

    /// Shows the window.
    pub fn show(&self) {
        unsafe { bind::SDL_ShowWindow(self.as_ptr()) }
    }

    /// Hides the window.
    pub fn hide(&self) {
        unsafe { bind::SDL_HideWindow(self.as_ptr()) }
    }

    /// Raises the window.
    pub fn raise(&self) {
        unsafe { bind::SDL_RaiseWindow(self.as_ptr()) }
    }

    /// Explicitly sets the input focus to the window.
    ///
    /// You almost want to use [`Window::raise`] instead of this. This might give focus to a window that is completely obscured by other windows.
    pub fn set_input_focus(&self) -> Result<()> {
        let ret = unsafe { bind::SDL_SetWindowInputFocus(self.as_ptr()) };
        if ret != 0 {
            return Err(crate::SdlError::Others { msg: Sdl::error() });
        }
        Ok(())
    }

    /// Make the window full screen.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to make it full screen.
    #[allow(clippy::unnecessary_cast)]
    pub fn full_screen(&self) -> Result<()> {
        let ret = unsafe {
            bind::SDL_SetWindowFullscreen(self.as_ptr(), bind::SDL_WINDOW_FULLSCREEN as u32)
        };
        if ret != 0 {
            return Err(crate::SdlError::Others { msg: Sdl::error() });
        }
        Ok(())
    }

    /// Maximizes the window.
    pub fn maximize(&self) {
        unsafe { bind::SDL_MaximizeWindow(self.as_ptr()) }
    }

    /// Minimizes the window.
    pub fn minimize(&self) {
        unsafe { bind::SDL_MinimizeWindow(self.as_ptr()) }
    }

    /// Restores the window from maximization/minimization.
    pub fn restore(&self) {
        unsafe { bind::SDL_RestoreWindow(self.as_ptr()) }
    }

    /// Sets an icon from a surface for the window.
    pub fn set_icon(&self, icon: &impl Surface) {
        unsafe { bind::SDL_SetWindowIcon(self.as_ptr(), icon.as_ptr().as_ptr()) }
    }

    /// Returns whether the window is showing the screen keyboard.
    #[must_use]
    pub fn is_screen_keyboard_shown(&self) -> bool {
        unsafe { bind::SDL_IsScreenKeyboardShown(self.as_ptr()) != 0 }
    }

    /// Makes the window surface.
    #[must_use]
    pub fn surface(&self) -> WindowSurface {
        WindowSurface::new(self)
    }

    fn sys_info(&self) -> bind::SDL_SysWMinfo {
        unsafe {
            let mut info = MaybeUninit::<bind::SDL_SysWMinfo>::uninit();
            let ptr = info.as_mut_ptr();
            bind::SDL_GetVersion(addr_of_mut!((*ptr).version));
            let ret = bind::SDL_GetWindowWMInfo(self.window.as_ptr(), ptr);
            if ret == 0 {
                panic!("failed to get window manager info: {}", Sdl::error());
            }
            info.assume_init()
        }
    }

    /// Gets a kind of the underlying subsystem.
    pub fn subsystem_kind(&self) -> SubsystemKind {
        let wm = self.sys_info();
        SubsystemKind::from_raw(wm.subsystem)
    }
}

impl<'video> Drop for Window<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyWindow(self.window.as_ptr()) }
    }
}

/// Supported windowing subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[non_exhaustive]
pub enum SubsystemKind {
    /// HWND handle for Windows.
    Windows,
    /// CoreWindow handle for Windows Runtime.
    WinRT,
    /// X11 Window handle for UNIX-like.
    X11,
    /// IDirectFBWindow handle for UNIX-like.
    DirectFB,
    /// NSWindow handle for macOS.
    Cocoa,
    /// UIWindow handle for iOS.
    UIKit,
    /// wl_display handle for UNIX-like.
    Wayland,
    /// ANativeWindow handle for Android.
    Android,
    /// EGLNativeWindowType handle for Vivante VDK (EGL).
    Vivante,
    /// HWND handle for OS/2.
    OS2,
    /// For Haiku OS.
    Haiku,
    /// Device index for libDRM.
    KmsDrm,
    /// For RISC OS.
    RiscOS,
}

impl SubsystemKind {
    pub(crate) fn from_raw(raw: bind::SDL_SYSWM_TYPE) -> Self {
        match raw {
            bind::SDL_SYSWM_WINDOWS => Self::Windows,
            bind::SDL_SYSWM_WINRT => Self::WinRT,
            bind::SDL_SYSWM_DIRECTFB => Self::DirectFB,
            bind::SDL_SYSWM_COCOA => Self::Cocoa,
            bind::SDL_SYSWM_UIKIT => Self::UIKit,
            bind::SDL_SYSWM_WAYLAND => Self::Wayland,
            bind::SDL_SYSWM_ANDROID => Self::Android,
            bind::SDL_SYSWM_VIVANTE => Self::Vivante,
            bind::SDL_SYSWM_OS2 => Self::OS2,
            bind::SDL_SYSWM_HAIKU => Self::Haiku,
            bind::SDL_SYSWM_KMSDRM => Self::KmsDrm,
            bind::SDL_SYSWM_RISCOS => Self::RiscOS,
            _ => panic!("unsupported subsystem: {}", raw),
        }
    }
}

unsafe impl<'video> HasRawWindowHandle for Window<'video> {
    /// Downcasts into a raw window handle.
    fn raw_window_handle(&self) -> RawWindowHandle {
        let wm = self.sys_info();
        let subsystem = SubsystemKind::from_raw(wm.subsystem);
        match subsystem {
            #[cfg(target_os = "windows")]
            SubsystemKind::Windows => {
                use raw_window_handle::windows::WindowsHandle;

                let mut handle = WindowsHandle::empty();
                handle.hwnd = unsafe { wm.info.win }.window as *mut c_void;
                RawWindowHandle::Windows(handle)
            }
            #[cfg(all(
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ),
                feature = "wayland"
            ))]
            SubsystemKind::Wayland => {
                use raw_window_handle::unix::WaylandHandle;

                let mut handle = WaylandHandle::empty();
                handle.surface = unsafe { wm.info.wl }.surface as *mut c_void;
                handle.display = unsafe { wm.info.wl }.display as *mut c_void;
                RawWindowHandle::Wayland(handle)
            }
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SubsystemKind::X11 => {
                use raw_window_handle::unix::XlibHandle;

                let mut handle = XlibHandle::empty();
                handle.window = unsafe { wm.info.x11 }.window;
                handle.display = unsafe { wm.info.x11 }.display as *mut c_void;
                RawWindowHandle::Xlib(handle)
            }
            #[cfg(target_os = "macos")]
            SubsystemKind::Cocoa => {
                use raw_window_handle::macos::MacOSHandle;

                let mut handle = MacOSHandle::empty();
                handle.ns_window = unsafe { wm.info.cocoa }.window.cast();
                RawWindowHandle::MacOS(handle)
            }
            #[cfg(target_os = "ios")]
            SubsystemKind::UIKit => {
                use raw_window_handle::ios::IOSHandle;

                let mut handle = IOSHandle::empty();
                handle.ui_window = unsafe { wm.info.uikit }.window.cast();
                RawWindowHandle::IOS(handle)
            }
            #[cfg(target_os = "android")]
            SubsystemKind::Android => {
                use raw_window_handle::android::AndroidHandle;

                let mut handle = AndroidHandle::empty();
                handle.a_native_window = unsafe { wm.info.android }.window.cast();
                RawWindowHandle::Android(handle)
            }
            _ => {
                panic!(
                    "unsupported window handle for this platform: {:?}",
                    subsystem
                );
            }
        }
    }
}
