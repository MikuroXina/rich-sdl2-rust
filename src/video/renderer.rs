use std::marker::PhantomData;
use std::ptr::NonNull;

use super::window::Window;

use crate::geo::Size;
use crate::{bind, Sdl};

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

    // TODO(MikuroXina): render target texture
    // TODO(MikuroXina): renderer info
    // TODO(MikuroXina): copy from texture
    // TODO(MikuroXina): clipping
    // TODO(MikuroXina): force-scaled by integer
    // TODO(MikuroXina): logical size
    // TODO(MikuroXina): scaling
    // TODO(MikuroXina): viewport

    // TODO(MikuroXina): texture mod
}

impl<'window> Drop for Renderer<'window> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyRenderer(self.renderer.as_ptr()) }
    }
}
