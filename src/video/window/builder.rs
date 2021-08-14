use std::ffi::CString;
use std::ptr::NonNull;

use super::{Window, WindowContextKind, WindowFlags, WindowFormat};
use crate::{bind, Sdl, Video};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPos {
    coord: i32,
}

impl WindowPos {
    pub fn coord(coord: i32) -> Self {
        const MAX: i32 = 16384;
        assert!(-MAX <= coord && coord <= MAX);
        Self { coord }
    }
    pub const fn undefined() -> Self {
        Self {
            coord: 0x1FFF0000, // SDL_WINDOWPOS_UNDEFINED
        }
    }
    pub const fn centered() -> Self {
        Self {
            coord: 0x2FFF0000, // SDL_WINDOWPOS_CENTERED
        }
    }
    pub fn into_arg(self) -> std::os::raw::c_int {
        self.coord
    }
}

#[derive(Debug)]
pub struct WindowBuilder {
    title: String,
    x: WindowPos,
    y: WindowPos,
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
            x: WindowPos::centered(),
            y: WindowPos::centered(),
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
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    pub fn x(mut self, x: WindowPos) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: WindowPos) -> Self {
        self.y = y;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn format(mut self, format: WindowFormat) -> Self {
        self.format = format;
        self
    }

    pub fn context_kind(mut self, context_kind: WindowContextKind) -> Self {
        self.context_kind = context_kind;
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn allow_high_dpi(mut self, allow_high_dpi: bool) -> Self {
        self.allow_high_dpi = allow_high_dpi;
        self
    }

    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

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
