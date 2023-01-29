#![allow(clippy::unnecessary_cast)]

use bitflags::bitflags;
use std::ffi::CString;
use std::ptr::NonNull;
use typed_builder::TypedBuilder;

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
    #[must_use]
    pub fn coord(coord: i32) -> Self {
        const MAX: i32 = 16384;
        assert!((-MAX..=MAX).contains(&coord));
        Self { coord }
    }
    /// Constructs the undefined coordinate.
    #[must_use]
    pub const fn undefined() -> Self {
        Self {
            coord: 0x1FFF0000, // SDL_WINDOWPOS_UNDEFINED
        }
    }
    /// Constructs the centered coordinate.
    #[must_use]
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
#[derive(Debug, TypedBuilder)]
pub struct WindowBuilder {
    #[builder(default = "Untitled".into(), setter(into))]
    title: String,
    #[builder(default = WindowCoord::centered())]
    x: WindowCoord,
    #[builder(default = WindowCoord::centered())]
    y: WindowCoord,
    #[builder(default = 640)]
    width: u32,
    #[builder(default = 480)]
    height: u32,
    #[builder(default = WindowFormat::Normal)]
    format: WindowFormat,
    #[builder(default = WindowContextKind::Software)]
    context_kind: WindowContextKind,
    #[builder(default)]
    hidden: bool,
    #[builder(default)]
    allow_high_dpi: bool,
    #[builder(default)]
    borderless: bool,
    #[builder(default)]
    resizable: bool,
}

impl WindowBuilder {
    /// Builds the window.
    #[must_use]
    pub fn new_window<'video>(self, video: &'video Video) -> Window<'video> {
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
            flags |= WindowFlags::BORDERLESS;
        }
        if self.resizable {
            flags |= WindowFlags::RESIZABLE;
        }
        flags.bits()
    }
}

bitflags! {
    /// A flag for [`Window`].
    pub(super) struct WindowFlags: u32 {
        const FULLSCREEN = bind::SDL_WINDOW_FULLSCREEN as u32;
        const FULLSCREEN_DESKTOP = bind::SDL_WINDOW_FULLSCREEN_DESKTOP as u32;
        const OPENGL = bind::SDL_WINDOW_OPENGL as u32;
        const VULKAN = bind::SDL_WINDOW_VULKAN as u32;
        const METAL = bind::SDL_WINDOW_METAL as u32;
        const SHOWN = bind::SDL_WINDOW_SHOWN as u32;
        const HIDDEN = bind::SDL_WINDOW_HIDDEN as u32;
        const BORDERLESS = bind::SDL_WINDOW_BORDERLESS as u32;
        const RESIZABLE = bind::SDL_WINDOW_RESIZABLE as u32;
        const MINIMIZED = bind::SDL_WINDOW_MINIMIZED as u32;
        const MAXIMIZED = bind::SDL_WINDOW_MAXIMIZED as u32;
        const INPUT_GRABBED = bind::SDL_WINDOW_INPUT_GRABBED as u32;
        const INPUT_FOCUS = bind::SDL_WINDOW_INPUT_FOCUS as u32;
        const MOUSE_FOCUS = bind::SDL_WINDOW_MOUSE_FOCUS as u32;
        const FOREIGN = bind::SDL_WINDOW_FOREIGN as u32;
        const ALLOW_HIGHDPI = bind::SDL_WINDOW_ALLOW_HIGHDPI as u32;
        const MOUSE_CAPTURE = bind::SDL_WINDOW_MOUSE_CAPTURE as u32;
    }
}
