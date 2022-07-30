//! OpenGL contexts and attributes.

use static_assertions::assert_not_impl_all;
use std::ffi::{c_void, CString};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::geo::Size;
use crate::window::{Window, WindowContextKind};
use crate::{bind, Result, Sdl, SdlError};

pub mod attribute;
mod buffer;
mod context_switch;
mod flag;

pub use buffer::*;
pub use context_switch::*;
pub use flag::*;

/// An OpenGL context controller.
pub struct GlContext<'window> {
    ctx: NonNull<c_void>,
    window: &'window Window<'window>,
}

impl std::fmt::Debug for GlContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlContext")
            .field("window", &self.window)
            .finish_non_exhaustive()
    }
}

assert_not_impl_all!(GlContext: Send, Sync);

impl<'window> GlContext<'window> {
    /// Constructs from a reference to [`Window`], or `Err` on failure.
    #[must_use]
    pub fn new(window: &'window Window) -> Option<Self> {
        if let WindowContextKind::OpenGl = window.state().context_kind {
            let raw = unsafe { bind::SDL_GL_CreateContext(window.as_ptr()) };
            NonNull::new(raw).map(|ctx| Self { ctx, window })
        } else {
            None
        }
    }

    /// Returns the internal pointer of the OpenGL context.
    #[must_use]
    pub fn as_ptr(&self) -> *mut c_void {
        self.ctx.as_ptr()
    }

    /// Returns the size that can be used to draw.
    pub fn drawable_size(&self) -> Size {
        let (mut width, mut height) = (MaybeUninit::uninit(), MaybeUninit::uninit());
        unsafe {
            bind::SDL_GL_GetDrawableSize(
                self.window.as_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
        }
        Size {
            width: unsafe { width.assume_init() } as u32,
            height: unsafe { height.assume_init() } as u32,
        }
    }

    /// Returns whether the extension `name` is supported.
    ///
    /// # Panics
    ///
    /// Panics if `name` contains a null character.
    #[must_use]
    pub fn supported_extension(&self, name: &'static str) -> bool {
        let cstr = CString::new(name).unwrap();
        unsafe { bind::SDL_GL_ExtensionSupported(cstr.as_ptr()) != 0 }
    }

    /// Loads the library from `path`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to load the library.
    ///
    /// # Panics
    ///
    /// Panics if `path` contains a null character.
    pub fn load_lib(&self, path: &str) -> Result<()> {
        let cstr = CString::new(path).unwrap();
        let ret = unsafe { bind::SDL_GL_LoadLibrary(cstr.as_ptr()) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(())
    }

    /// Unloads all of loaded libraries.
    pub fn unload_lib_all(&self) {
        unsafe { bind::SDL_GL_UnloadLibrary() }
    }

    /// Returns the raw address of the procedure.
    ///
    /// # Safety
    ///
    /// This return value is valid only on supported the extension.
    /// You must check by `supported_extension` before casting to any function pointer.
    ///
    /// # Panics
    ///
    /// Panics if `proc` contains a null character.
    #[must_use]
    pub unsafe fn proc_address(&self, proc: &str) -> *mut c_void {
        let cstr = CString::new(proc).unwrap();
        unsafe { bind::SDL_GL_GetProcAddress(cstr.as_ptr()) }
    }
}

impl<'window> Drop for GlContext<'window> {
    fn drop(&mut self) {
        unsafe {
            bind::SDL_GL_UnloadLibrary();
            bind::SDL_GL_DeleteContext(self.ctx.as_ptr());
        }
    }
}
