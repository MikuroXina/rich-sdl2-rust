//! Renderer for a window, to render some geometries or query driver information.

use static_assertions::assert_not_impl_all;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use clip::ClippedRenderer;

use super::window::Window;
use crate::{
    bind,
    geo::{Rect, Scale, Size},
    texture::Texture,
    Result, Sdl, SdlError,
};

pub mod clip;
pub mod driver;
pub mod info;
mod paste;
pub mod pen;

pub use paste::*;

/// A SDL2 renderer. This is often used for rendering with [`pen::Pen`].
pub struct Renderer<'window> {
    renderer: NonNull<bind::SDL_Renderer>,
    window: &'window Window<'window>,
}

impl std::fmt::Debug for Renderer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer")
            .field("window", &self.window)
            .finish_non_exhaustive()
    }
}

assert_not_impl_all!(Renderer: Send, Sync);

impl<'window> Renderer<'window> {
    /// Constructs a renderer from the window.
    #[must_use]
    pub fn new(window: &'window Window) -> Self {
        let raw = unsafe { bind::SDL_CreateRenderer(window.as_ptr(), -1, 0) };
        NonNull::new(raw).map_or_else(
            || Sdl::error_then_panic("Sdl renderer"),
            |renderer| Self { renderer, window },
        )
    }

    pub(crate) fn as_ptr(&self) -> *mut bind::SDL_Renderer {
        self.renderer.as_ptr()
    }

    /// Returns the borrowing window.
    #[must_use]
    pub fn window(&self) -> &Window {
        self.window
    }

    /// Returns the geometry size of the output from the renderer.
    pub fn output_size(&self) -> Result<Size> {
        let (mut w, mut h) = (0i32, 0i32);
        let ret = unsafe { bind::SDL_GetRendererOutputSize(self.as_ptr(), &mut w, &mut h) };
        if ret == 0 {
            Err(SdlError::Others {
                msg: "Getting output size failed".into(),
            })
        } else {
            Ok(Size {
                width: w as u32,
                height: h as u32,
            })
        }
    }

    /// Clips the renderer by `area`.
    pub fn clip(&'window mut self, area: Rect) -> ClippedRenderer<'window> {
        ClippedRenderer::new(self, area)
    }

    /// Returns the logical size of the renderer if available.
    #[must_use]
    pub fn logical_size(&self) -> Option<Size> {
        let (mut width, mut height) = (0, 0);
        unsafe { bind::SDL_RenderGetLogicalSize(self.as_ptr(), &mut width, &mut height) }
        if width == 0 && height == 0 {
            return None;
        }
        Some(Size {
            width: width as u32,
            height: height as u32,
        })
    }

    /// Sets the logical size of the renderer.
    pub fn set_logical_size(&self, Size { width, height }: Size) {
        use std::os::raw::c_int;
        let ret = unsafe {
            bind::SDL_RenderSetLogicalSize(self.as_ptr(), width as c_int, height as c_int)
        };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer logical size");
        }
    }

    /// Returns whether integer scaled is forced.
    #[must_use]
    pub fn is_forced_integer_scale(&self) -> bool {
        unsafe { bind::SDL_RenderGetIntegerScale(self.as_ptr()) != 0 }
    }

    /// Sets whether integer scaled is forced.
    pub fn force_integer_scale(&self, enabled: bool) {
        let ret =
            unsafe { bind::SDL_RenderSetIntegerScale(self.as_ptr(), if enabled { 1 } else { 0 }) };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer integer scale");
        }
    }

    /// Returns the scale of th renderer.
    pub fn scale(&self) -> Scale {
        let mut scale = Scale {
            horizontal: 0.0,
            vertical: 0.0,
        };
        unsafe {
            bind::SDL_RenderGetScale(self.as_ptr(), &mut scale.horizontal, &mut scale.vertical);
        }
        scale
    }

    /// Sets the scale of the renderer.
    pub fn set_scale(
        &self,
        Scale {
            horizontal,
            vertical,
        }: Scale,
    ) {
        let ret = unsafe { bind::SDL_RenderSetScale(self.as_ptr(), horizontal, vertical) };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer scale");
        }
    }

    /// Returns the viewport rectangle of the renderer.
    pub fn viewport(&self) -> Rect {
        let mut raw_rect = MaybeUninit::uninit();
        unsafe { bind::SDL_RenderGetViewport(self.as_ptr(), raw_rect.as_mut_ptr()) }
        unsafe { raw_rect.assume_init() }.into()
    }

    /// Sets the viewport rectangle of the renderer.
    pub fn set_viewport(&self, area: Option<Rect>) {
        let ret = unsafe {
            bind::SDL_RenderSetViewport(
                self.as_ptr(),
                area.map_or(std::ptr::null(), |rect| &rect.into()),
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer viewport");
        }
    }

    /// Sets the render target to the texture.
    pub fn set_target<'texture: 'window>(&'window self, texture: &'texture Texture) {
        let ret = unsafe { bind::SDL_SetRenderTarget(self.as_ptr(), texture.as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer target texture");
        }
    }

    /// Resets the render target to the original window.
    pub fn set_target_default(&self) {
        let ret = unsafe { bind::SDL_SetRenderTarget(self.as_ptr(), std::ptr::null_mut()) };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer target default");
        }
    }
}

impl<'window> Drop for Renderer<'window> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyRenderer(self.renderer.as_ptr()) }
    }
}
