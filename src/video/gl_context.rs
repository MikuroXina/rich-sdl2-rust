use std::ffi::{c_void, CString};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::geo::Size;
use crate::window::{Window, WindowContextKind};
use crate::{bind, Result, Sdl, SdlError};

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

    pub fn supported_extension(&self, name: &'static str) -> bool {
        let cstr = CString::new(name).unwrap();
        unsafe { bind::SDL_GL_ExtensionSupported(cstr.as_ptr()) != 0 }
    }

    pub fn load_lib(&self, path: &str) -> Result<()> {
        let cstr = CString::new(path).unwrap();
        let ret = unsafe { bind::SDL_GL_LoadLibrary(cstr.as_ptr()) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(())
    }

    // TODO(MikuroXina): proc
}

impl<'window> Drop for GlContext<'window> {
    fn drop(&mut self) {
        unsafe {
            bind::SDL_GL_UnloadLibrary();
            bind::SDL_GL_DeleteContext(self.ctx.as_ptr())
        }
    }
}
