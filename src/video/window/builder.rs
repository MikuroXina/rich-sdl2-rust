use bitflags::bitflags;
use std::ffi::CString;
use std::ptr::NonNull;

use super::{Window, WindowContextKind, WindowFormat};
use crate::{bind, Sdl, Video};

/// A coordinate value for the window position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCoord {
    coord: i32,
}

impl WindowCoord {
    /// Constructs from coord value. Must be in `-16384..=16384`.
    ///
    /// # Panics
    ///
    /// Panics if `coord` is not in `-16384..=16384`.
    pub fn coord(coord: i32) -> Self {
        const MAX: i32 = 16384;
        assert!(-MAX <= coord && coord <= MAX);
        Self { coord }
    }
    /// Constructs the undefined coordinate.
    pub const fn undefined() -> Self {
        Self {
            coord: 0x1FFF0000, // SDL_WINDOWPOS_UNDEFINED
        }
    }
    /// Constructs the centered coordinate.
    pub const fn centered() -> Self {
        Self {
            coord: 0x2FFF0000, // SDL_WINDOWPOS_CENTERED
        }
    }
    pub(super) fn into_arg(self) -> std::os::raw::c_int {
        self.coord
    }
}

/// A builder for the [`Window`].
#[derive(Debug)]
pub struct WindowBuilder {
    title: String,
    x: WindowCoord,
    y: WindowCoord,
    width: u32,
    height: u32,
    format: WindowFormat,
    context_kind: WindowContextKind,
    hidden: bool,
    allow_high_dpi: bool,
    borderless: bool,
    resizable: bool,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            title: "Untitled".into(),
            x: WindowCoord::centered(),
            y: WindowCoord::centered(),
            width: 640,
            height: 480,
            format: WindowFormat::Normal,
            context_kind: WindowContextKind::Software,
            hidden: false,
            allow_high_dpi: false,
            borderless: false,
            resizable: false,
        }
    }
}

impl WindowBuilder {
    /// Sets the title of the window.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    /// Sets the x coordinate of the window.
    pub fn x(mut self, x: WindowCoord) -> Self {
        self.x = x;
        self
    }

    /// Sets the y coordinate of the window.
    pub fn y(mut self, y: WindowCoord) -> Self {
        self.y = y;
        self
    }

    /// Sets the width of the window.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the window.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Sets the format of the window.
    pub fn format(mut self, format: WindowFormat) -> Self {
        self.format = format;
        self
    }

    /// Sets the context kind of the window.
    pub fn context_kind(mut self, context_kind: WindowContextKind) -> Self {
        self.context_kind = context_kind;
        self
    }

    /// Sets whether the window is hidden.
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Sets whether the window allows high dpi.
    pub fn allow_high_dpi(mut self, allow_high_dpi: bool) -> Self {
        self.allow_high_dpi = allow_high_dpi;
        self
    }

    /// Sets whether the window is borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Sets whether the window is resizable.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Builds the window.
    pub fn build<'video>(self, video: &'video Video) -> Window<'video> {
        if self.context_kind == WindowContextKind::Vulkan {
            let ret = unsafe { bind::SDL_Vulkan_LoadLibrary(std::ptr::null()) };
            if ret == -1 {
                Sdl::error_then_panic("loading vulkan library from SDL_VULKAN_LIBRARY");
            }
        }

        let flags = self.calc_flags();

        use std::os::raw::c_int;
        let cstr = CString::new(self.title).unwrap_or_default();
        let raw = unsafe {
            bind::SDL_CreateWindow(
                cstr.as_ptr(),
                self.x.into_arg(),
                self.y.into_arg(),
                self.width as c_int,
                self.height as c_int,
                flags,
            )
        };
        NonNull::new(raw).map_or_else(
            || Sdl::error_then_panic("Sdl window"),
            move |window| Window { window, video },
        )
    }

    fn calc_flags(&self) -> u32 {
        let mut flags = WindowFlags::empty();
        flags |= match self.format {
            WindowFormat::Normal => WindowFlags::empty(),
            WindowFormat::Maximized => WindowFlags::MAXIMIZED,
            WindowFormat::Minimized => WindowFlags::MINIMIZED,
            WindowFormat::FullScreen => WindowFlags::FULLSCREEN,
            WindowFormat::FullScreenWithCurrentDesktop => WindowFlags::FULLSCREEN_DESKTOP,
        };
        flags |= match self.context_kind {
            WindowContextKind::Software => WindowFlags::empty(),
            WindowContextKind::OpenGl => WindowFlags::OPENGL,
            WindowContextKind::Vulkan => WindowFlags::VULKAN,
            WindowContextKind::Metal => WindowFlags::METAL,
        };
        if self.hidden {
            flags |= WindowFlags::HIDDEN;
        }
        if self.allow_high_dpi {
            flags |= WindowFlags::ALLOW_HIGHDPI;
        }
        if self.borderless {
            flags |= WindowFlags::BORDERLESS
        }
        flags.bits()
    }
}

bitflags! {
    /// A flag for [`Window`].
    pub(super) struct WindowFlags: u32 {
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
