use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;

use crate::geo::{Point, Size};
use crate::{bind, Result, SdlError};

use super::{Window, WindowPos};

pub struct Position {
    pub x: WindowPos,
    pub y: WindowPos,
}

pub struct Opacity {
    opacity: f32,
}

impl Opacity {
    pub fn new(opacity: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&opacity) {
            return None;
        }
        Some(Self { opacity })
    }

    pub fn as_f32(&self) -> f32 {
        self.opacity
    }
}

pub trait ConfigExt {
    fn max_size(&self) -> Size;
    fn min_size(&self) -> Size;
    fn size(&self) -> Size;
    fn opacity(&self) -> Opacity;
    fn pos(&self) -> Point;
    fn title(&self) -> &str;

    fn set_max_size(&self, max_size: Size);
    fn set_min_size(&self, min_size: Size);
    fn set_size(&self, size: Size);
    fn set_opacity(&self, opacity: Opacity) -> Result<()>;
    fn set_pos(&self, pos: Position);
    fn set_title(&self, title: &str);
    fn set_resizable(&self, resizable: bool);

    fn add_frame(&self);
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
