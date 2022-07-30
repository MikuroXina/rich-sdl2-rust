//! Metal API support in SDL2.

use std::{ffi::c_void, ptr::NonNull};

use crate::{bind, geo::Size, window::Window};

/// A view in Metal API, made from a window.
pub struct MetalView<'window> {
    ptr: NonNull<c_void>,
    window: &'window Window<'window>,
}

impl<'window> MetalView<'window> {
    /// Constructs a Metal view.
    #[must_use]
    pub fn new(window: &'window Window<'window>) -> Self {
        let ptr = unsafe { bind::SDL_Metal_CreateView(window.as_ptr()) };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
            window,
        }
    }

    /// Returns the raw pointer `CAMetalLayer` from the view.
    #[must_use]
    pub fn metal_layer(&self) -> *mut c_void {
        unsafe { bind::SDL_Metal_GetLayer(self.ptr.as_ptr()) }
    }

    /// Returns the drawable size of the view.
    pub fn drawable_size(&self) -> Size {
        let mut width = 0;
        let mut height = 0;
        unsafe {
            bind::SDL_Metal_GetDrawableSize(
                self.window.as_ptr(),
                &mut width as *mut _,
                &mut height as *mut _,
            )
        }
        Size {
            width: width as _,
            height: height as _,
        }
    }
}

impl Drop for MetalView<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_Metal_DestroyView(self.ptr.as_ptr()) }
    }
}
