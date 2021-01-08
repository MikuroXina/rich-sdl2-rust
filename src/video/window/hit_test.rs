use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::Sdl;
use crate::{bind, geo::Point, Result};

use super::Window;

pub enum HitTestResult {
    Normal,
    Draggable,
    ResizeTopLeft,
    ResizeTop,
    ResizeTopRight,
    ResizeRight,
    ResizeBottomRight,
    ResizeBottom,
    ResizeBottomLeft,
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

pub type HitTester = Box<dyn Fn(Point) -> HitTestResult>;

thread_local! {
    static HIT_TESTERS: RefCell<HashMap<u32, HitTester>> = RefCell::new(HashMap::new());
}

pub struct HitTest<'window> {
    window: &'window Window<'window>,
}

impl<'window> HitTest<'window> {
    pub fn new(window: &'window Window<'window>) -> Self {
        Self { window }
    }

    pub fn set_tester(&self, callback: HitTester) -> Result<()> {
        HIT_TESTERS.with(|tester| tester.borrow_mut().insert(self.window.id(), callback));
        let ret = unsafe {
            bind::SDL_SetWindowHitTest(self.window.as_ptr(), Some(handler), std::ptr::null_mut())
        };
        if ret != 0 {
            return Err(crate::SdlError::UnsupportedFeature { msg: Sdl::error() });
        }
        Ok(())
    }
}

impl Drop for HitTest<'_> {
    fn drop(&mut self) {
        HIT_TESTERS.with(|tester| tester.borrow_mut().remove(&self.window.id()));
    }
}

unsafe extern "C" fn handler(
    win: *mut bind::SDL_Window,
    area: *const bind::SDL_Point,
    _data: *mut ::std::os::raw::c_void,
) -> bind::SDL_HitTestResult {
    let result = Cell::new(0u32);
    HIT_TESTERS.with(|f| {
        let id = bind::SDL_GetWindowID(win);
        if let Some(hit_tester) = f.borrow().get(&id) {
            result.set(hit_tester((*area).into()).into_arg());
        }
    });
    result.get()
}
