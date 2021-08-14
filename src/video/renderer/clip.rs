use crate::geo::Rect;
use crate::{bind, Sdl};

use super::Renderer;

#[derive(Debug)]
pub struct ClippedRenderer<'renderer> {
    renderer: &'renderer mut Renderer<'renderer>,
}

impl<'renderer> ClippedRenderer<'renderer> {
    pub(super) fn new(renderer: &'renderer mut Renderer<'renderer>, rect: Rect) -> Self {
        let ret =
            unsafe { bind::SDL_RenderSetClipRect(renderer.as_ptr(), &rect.into() as *const _) };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer clip rect");
        }
        Self { renderer }
    }
}

impl Drop for ClippedRenderer<'_> {
    fn drop(&mut self) {
        let ret = unsafe { bind::SDL_RenderSetClipRect(self.renderer.as_ptr(), std::ptr::null()) };
        debug_assert!(ret == 0);
    }
}
