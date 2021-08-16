use static_assertions::assert_not_impl_all;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ffi::c_void,
};

use crate::{bind, geo::Point, Result};

use super::Window;

/// A hit test result that determined cursor from the mouse position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitTestResult {
    /// The cursor should be normal.
    Normal,
    /// The cursor should be draggable.
    Draggable,
    /// The cursor should be resizing on the top left.
    ResizeTopLeft,
    /// The cursor should be resizing on the top.
    ResizeTop,
    /// The cursor should be resizing on the top right.
    ResizeTopRight,
    /// The cursor should be resizing on the right.
    ResizeRight,
    /// The cursor should be resizing on the bottom right.
    ResizeBottomRight,
    /// The cursor should be resizing on the bottom.
    ResizeBottom,
    /// The cursor should be resizing on the bottom left.
    ResizeBottomLeft,
    /// The cursor should be resizing on the left.
    ResizeLeft,
}

impl HitTestResult {
    fn into_arg(self) -> bind::SDL_HitTestResult {
        use HitTestResult::*;
        match self {
            Normal => bind::SDL_HitTestResult_SDL_HITTEST_NORMAL,
            Draggable => bind::SDL_HitTestResult_SDL_HITTEST_DRAGGABLE,
            ResizeTopLeft => bind::SDL_HitTestResult_SDL_HITTEST_RESIZE_TOPLEFT,
            ResizeTop => bind::SDL_HitTestResult_SDL_HITTEST_RESIZE_TOP,
            ResizeTopRight => bind::SDL_HitTestResult_SDL_HITTEST_RESIZE_TOPRIGHT,
            ResizeRight => bind::SDL_HitTestResult_SDL_HITTEST_RESIZE_RIGHT,
            ResizeBottomRight => bind::SDL_HitTestResult_SDL_HITTEST_RESIZE_BOTTOMRIGHT,
            ResizeBottom => bind::SDL_HitTestResult_SDL_HITTEST_RESIZE_BOTTOM,
            ResizeBottomLeft => bind::SDL_HitTestResult_SDL_HITTEST_RESIZE_BOTTOMLEFT,
            ResizeLeft => bind::SDL_HitTestResult_SDL_HITTEST_RESIZE_LEFT,
        }
    }
}

/// A callback to hit test with the mouse position.
pub type HitTester<'window> = Box<dyn FnMut(Point) -> HitTestResult + 'window>;

/// A hit tester for a window.
#[derive(Debug)]
pub struct HitTest<'window> {
    window: &'window Window<'window>,
    tester_raw: *mut HitTester<'window>,
}

assert_not_impl_all!(HitTest: Send, Sync);

impl<'window> HitTest<'window> {
    /// Constructs a hit test from the window and a callback.
    pub fn new(window: &'window Window<'window>, callback: HitTester<'window>) -> Result<Self> {
        let wrapped = Box::new(callback);
        let tester_raw = Box::into_raw(wrapped);
        let ret = unsafe {
            bind::SDL_SetWindowHitTest(
                window.as_ptr(),
                Some(hit_test_wrap_handler),
                tester_raw.cast(),
            )
        };
        if ret != 0 {
            Err(crate::SdlError::UnsupportedFeature)
        } else {
            Ok(Self { window, tester_raw })
        }
    }
}

impl Drop for HitTest<'_> {
    fn drop(&mut self) {
        let _ =
            unsafe { bind::SDL_SetWindowHitTest(self.window.as_ptr(), None, std::ptr::null_mut()) };
        let _ = unsafe { Box::from_raw(self.tester_raw) };
    }
}

unsafe extern "C" fn hit_test_wrap_handler(
    win: *mut bind::SDL_Window,
    area: *const bind::SDL_Point,
    data: *mut c_void,
) -> bind::SDL_HitTestResult {
    let callback = unsafe { &mut *(data as *mut HitTester) };
    callback((*area).into()).into_arg()
}
