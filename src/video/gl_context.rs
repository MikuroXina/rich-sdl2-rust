use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::geo::Size;
use crate::window::WindowContextKind;
use crate::{bind, window::Window};

pub mod attribute;
mod buffer;
mod context_switch;

pub use buffer::*;
pub use context_switch::*;

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

    pub fn drawable_size(&self) -> Size {
        let (mut width, mut height) = (MaybeUninit::uninit(), MaybeUninit::uninit());
        unsafe {
            bind::SDL_GL_GetDrawableSize(
                self.window.as_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            )
        }
        Size {
            width: unsafe { width.assume_init() } as u32,
            height: unsafe { height.assume_init() } as u32,
        }
    }

    // TODO(MikuroXina): library
    // TODO(MikuroXina): proc
    // TODO(MikuroXina): extension
}

impl<'window> Drop for GlContext<'window> {
    fn drop(&mut self) {
        unsafe { bind::SDL_GL_DeleteContext(self.ctx.as_ptr()) }
    }
}
