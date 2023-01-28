use static_assertions::assert_not_impl_all;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ffi::c_void,
    ptr::addr_of_mut,
};

use crate::{bind, geo::Point, Result};

use super::Window;

/// A hit test result that determined cursor from the mouse position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
        match self {
            HitTestResult::Normal => bind::SDL_HITTEST_NORMAL,
            HitTestResult::Draggable => bind::SDL_HITTEST_DRAGGABLE,
            HitTestResult::ResizeTopLeft => bind::SDL_HITTEST_RESIZE_TOPLEFT,
            HitTestResult::ResizeTop => bind::SDL_HITTEST_RESIZE_TOP,
            HitTestResult::ResizeTopRight => bind::SDL_HITTEST_RESIZE_TOPRIGHT,
            HitTestResult::ResizeRight => bind::SDL_HITTEST_RESIZE_RIGHT,
            HitTestResult::ResizeBottomRight => bind::SDL_HITTEST_RESIZE_BOTTOMRIGHT,
            HitTestResult::ResizeBottom => bind::SDL_HITTEST_RESIZE_BOTTOM,
            HitTestResult::ResizeBottomLeft => bind::SDL_HITTEST_RESIZE_BOTTOMLEFT,
            HitTestResult::ResizeLeft => bind::SDL_HITTEST_RESIZE_LEFT,
        }
    }
}

/// A callback to hit test with the mouse position.
pub trait HitTester<'window>: FnMut(Point) -> HitTestResult + 'window {}

/// A hit tester for a window.
#[derive(Debug)]
pub struct HitTest<'window, T> {
    window: &'window Window<'window>,
    tester: T,
}

impl<'window, T: HitTester<'window>> HitTest<'window, T> {
    /// Constructs a hit test from the window and a callback.
    ///
    /// # Errors
    ///
    /// Returns `Err` if a hit test in the window is unsupported.
    pub fn new(window: &'window Window<'window>, mut tester: T) -> Result<Self> {
        let data = addr_of_mut!(tester);
        let ret = unsafe {
            bind::SDL_SetWindowHitTest(
                window.as_ptr(),
                Some(hit_test_wrap_handler::<T>),
                data.cast(),
            )
        };
        if ret == 0 {
            Ok(Self { window, tester })
        } else {
            Err(crate::SdlError::UnsupportedFeature)
        }
    }
}

impl<T> Drop for HitTest<'_, T> {
    fn drop(&mut self) {
        let _ =
            unsafe { bind::SDL_SetWindowHitTest(self.window.as_ptr(), None, std::ptr::null_mut()) };
    }
}

unsafe extern "C" fn hit_test_wrap_handler<'window, T: HitTester<'window>>(
    win: *mut bind::SDL_Window,
    area: *const bind::SDL_Point,
    data: *mut c_void,
) -> bind::SDL_HitTestResult {
    let callback = unsafe { &mut *data.cast::<T>() };
    callback((*area).into()).into_arg()
}
