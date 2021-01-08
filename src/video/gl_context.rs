use std::ffi::c_void;
use std::ptr::NonNull;

use crate::window::WindowContextKind;
use crate::{bind, window::Window};

pub struct GlContext<'window> {
    ctx: NonNull<c_void>,
    window: &'window Window<'window>,
}

impl<'window> GlContext<'window> {
    pub fn new(window: &'window Window) -> Option<Self> {
        if let WindowContextKind::OpenGl = window.state().context_kind {
            let raw = unsafe { bind::SDL_GL_CreateContext(window.as_ptr()) };
            NonNull::new(raw).map(|ctx| Self { ctx, window })
        } else {
            None
        }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ctx.as_ptr()
    }
}

impl<'window> Drop for GlContext<'window> {
    fn drop(&mut self) {
        unsafe { bind::SDL_GL_DeleteContext(self.ctx.as_ptr()) }
    }
}
