use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;

use crate::geo::{Point, Size};
use crate::{bind, Result, SdlError};

use super::{Window, WindowCoord};

/// A position of the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The x coordinate of the window.
    pub x: WindowCoord,
    /// The y coordinate of the window.
    pub y: WindowCoord,
}

/// An opacity of the window.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Opacity {
    opacity: f32,
}

impl Opacity {
    /// Constructs from opacity, or `None` if the value is not in `0.0..=1.0`.
    #[must_use]
    pub fn new(opacity: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&opacity) {
            return None;
        }
        Some(Self { opacity })
    }

    /// Constructs from opacity, clamping to `0.0,,=1.0`.
    #[must_use]
    pub fn with_clamped(opacity: f32) -> Self {
        Self {
            opacity: opacity.clamp(0.0, 1.0),
        }
    }

    /// Converts into `f32`
    #[must_use]
    pub fn as_f32(&self) -> f32 {
        self.opacity
    }
}

/// An extension for [`Window`] to configure its properties.
pub trait ConfigExt {
    /// Returns the maximum size of the window.
    fn max_size(&self) -> Size;
    /// Returns the minimum size of the window.
    fn min_size(&self) -> Size;
    /// Returns the current size of the window.
    fn size(&self) -> Size;
    /// Returns the opacity of the window.
    fn opacity(&self) -> Opacity;
    /// Returns the position of the window.
    fn pos(&self) -> Point;
    /// Returns the title of the window.
    fn title(&self) -> &str;

    /// Sets the maximum size of the window.
    fn set_max_size(&self, max_size: Size);
    /// Sets the minimum size of the window.
    fn set_min_size(&self, min_size: Size);
    /// Sets the current size of the window.
    fn set_size(&self, size: Size);
    /// Sets the opacity of the window.
    fn set_opacity(&self, opacity: Opacity) -> Result<()>;
    /// Sets the position of the window.
    fn set_pos(&self, pos: Position);
    /// Sets the title of the window.
    fn set_title(&self, title: &str);
    /// Sets whether the window is resizable.
    fn set_resizable(&self, resizable: bool);

    /// Adds a frame to the window.
    fn add_frame(&self);
    /// Removes a frame from the window.
    fn remove_frame(&self);
}

impl ConfigExt for Window<'_> {
    fn max_size(&self) -> Size {
        let (mut width, mut height) = (MaybeUninit::uninit(), MaybeUninit::uninit());
        unsafe {
            bind::SDL_GetWindowMaximumSize(self.as_ptr(), width.as_mut_ptr(), height.as_mut_ptr())
        }
        Size {
            width: unsafe { width.assume_init() } as _,
            height: unsafe { height.assume_init() } as _,
        }
    }

    fn min_size(&self) -> Size {
        let (mut width, mut height) = (MaybeUninit::uninit(), MaybeUninit::uninit());
        unsafe {
            bind::SDL_GetWindowMinimumSize(self.as_ptr(), width.as_mut_ptr(), height.as_mut_ptr())
        }
        Size {
            width: unsafe { width.assume_init() } as _,
            height: unsafe { height.assume_init() } as _,
        }
    }

    fn size(&self) -> Size {
        let (mut width, mut height) = (MaybeUninit::uninit(), MaybeUninit::uninit());
        unsafe { bind::SDL_GetWindowSize(self.as_ptr(), width.as_mut_ptr(), height.as_mut_ptr()) }
        Size {
            width: unsafe { width.assume_init() } as _,
            height: unsafe { height.assume_init() } as _,
        }
    }

    fn opacity(&self) -> Opacity {
        let mut opacity = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetWindowOpacity(self.as_ptr(), opacity.as_mut_ptr()) };
        let opacity = if ret != 0 {
            1.0
        } else {
            unsafe { opacity.assume_init() }
        };
        Opacity { opacity }
    }

    fn pos(&self) -> Point {
        let mut point = Point::default();
        unsafe {
            bind::SDL_GetWindowPosition(
                self.as_ptr(),
                &mut point.x as *mut _,
                &mut point.y as *mut _,
            )
        }
        point
    }

    fn title(&self) -> &str {
        let raw = unsafe { bind::SDL_GetWindowTitle(self.as_ptr()) };
        unsafe { CStr::from_ptr(raw) }
            .to_str()
            .expect("Getting title failed")
    }

    fn set_max_size(&self, Size { width, height }: Size) {
        unsafe { bind::SDL_SetWindowMaximumSize(self.as_ptr(), width as _, height as _) }
    }

    fn set_min_size(&self, Size { width, height }: Size) {
        unsafe { bind::SDL_SetWindowMinimumSize(self.as_ptr(), width as _, height as _) }
    }

    fn set_size(&self, Size { width, height }: Size) {
        unsafe { bind::SDL_SetWindowSize(self.as_ptr(), width as _, height as _) }
    }

    fn set_opacity(&self, opacity: Opacity) -> Result<()> {
        let ret = unsafe { bind::SDL_SetWindowOpacity(self.as_ptr(), opacity.opacity) };
        if ret != 0 {
            return Err(SdlError::UnsupportedFeature);
        }
        Ok(())
    }

    fn set_pos(&self, Position { x, y }: Position) {
        unsafe { bind::SDL_SetWindowPosition(self.as_ptr(), x.into_arg(), y.into_arg()) }
    }

    fn set_title(&self, title: &str) {
        let cstr = CString::new(title).unwrap_or_default();
        unsafe { bind::SDL_SetWindowTitle(self.as_ptr(), cstr.as_ptr()) }
    }

    fn set_resizable(&self, resizable: bool) {
        unsafe { bind::SDL_SetWindowResizable(self.as_ptr(), if resizable { 1 } else { 0 }) }
    }

    fn add_frame(&self) {
        unsafe { bind::SDL_SetWindowBordered(self.as_ptr(), 1) }
    }

    fn remove_frame(&self) {
        unsafe { bind::SDL_SetWindowBordered(self.as_ptr(), 0) }
    }
}
