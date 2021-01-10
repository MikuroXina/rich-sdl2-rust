use std::marker::PhantomData;
use std::ptr::NonNull;

use clip::ClippedRenderer;

use super::window::Window;

use crate::geo::{Rect, Scale, Size};
use crate::{bind, Sdl};

pub mod clip;
pub mod info;
pub mod pen;

pub enum BlendMode {
    None,
    AlphaBlend,
    Add,
    Mul,
}

impl From<bind::SDL_BlendMode> for BlendMode {
    fn from(raw: bind::SDL_BlendMode) -> Self {
        use BlendMode::*;
        match raw {
            bind::SDL_BlendMode_SDL_BLENDMODE_BLEND => AlphaBlend,
            bind::SDL_BlendMode_SDL_BLENDMODE_ADD => Add,
            bind::SDL_BlendMode_SDL_BLENDMODE_MOD => Mul,
            _ => None,
        }
    }
}

pub struct Renderer<'window> {
    renderer: NonNull<bind::SDL_Renderer>,
    _phantom: PhantomData<&'window ()>,
}

impl<'window> Renderer<'window> {
    pub fn new(window: &'window Window) -> Self {
        let raw = unsafe { bind::SDL_CreateRenderer(window.as_ptr(), -1, 0) };
        NonNull::new(raw).map_or_else(
            || Sdl::error_then_panic("Sdl renderer"),
            |renderer| Self {
                renderer,
                _phantom: PhantomData,
            },
        )
    }

    pub fn as_ptr(&self) -> *mut bind::SDL_Renderer {
        self.renderer.as_ptr()
    }

    pub fn output_size(&self) -> Size {
        let (mut w, mut h) = (0i32, 0i32);
        let ret = unsafe {
            bind::SDL_GetRendererOutputSize(self.as_ptr(), &mut w as *mut _, &mut h as *mut _)
        };
        if ret != 0 {
            panic!("Getting output size failed");
        }
        Size {
            width: w as u32,
            height: h as u32,
        }
    }

    pub fn blend_mode(&self) -> BlendMode {
        let mut raw = 0;
        let ret = unsafe { bind::SDL_GetRenderDrawBlendMode(self.as_ptr(), &mut raw as *mut _) };
        if ret != 0 {
            Sdl::error_then_panic("Getting renderer blend mode");
        }
        raw.into()
    }

    pub fn clip(&'window mut self, area: Rect) -> ClippedRenderer<'window> {
        ClippedRenderer::new(self, area)
    }

    pub fn logical_size(&self) -> Option<Size> {
        let (mut width, mut height) = (0, 0);
        unsafe {
            bind::SDL_RenderGetLogicalSize(
                self.as_ptr(),
                &mut width as *mut _,
                &mut height as *mut _,
            )
        }
        if width == 0 && height == 0 {
            return None;
        }
        Some(Size {
            width: width as u32,
            height: height as u32,
        })
    }

    pub fn set_logical_size(&self, Size { width, height }: Size) {
        use std::os::raw::c_int;
        let ret = unsafe {
            bind::SDL_RenderSetLogicalSize(self.as_ptr(), width as c_int, height as c_int)
        };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer logical size");
        }
    }

    pub fn is_forced_integer_scale(&self) -> bool {
        unsafe { bind::SDL_RenderGetIntegerScale(self.as_ptr()) != 0 }
    }

    pub fn force_integer_scale(&self, enabled: bool) {
        let ret =
            unsafe { bind::SDL_RenderSetIntegerScale(self.as_ptr(), if enabled { 1 } else { 0 }) };
        if ret != 0 {
            Sdl::error_then_panic("Setting renderer integer scale");
        }
    }

    pub fn scale(&self) -> Scale {
        let mut scale = Scale {
            horizontal: 0.0,
            vertical: 0.0,
        };
        unsafe {
            bind::SDL_RenderGetScale(
                self.as_ptr(),
                &mut scale.horizontal as *mut _,
                &mut scale.vertical as *mut _,
            )
        }
        scale
    }

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

    // TODO(MikuroXina): render target texture
    // TODO(MikuroXina): copy from texture
    // TODO(MikuroXina): viewport

    // TODO(MikuroXina): texture mod
}

impl<'window> Drop for Renderer<'window> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyRenderer(self.renderer.as_ptr()) }
    }
}
