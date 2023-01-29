//! Defining information structures of a renderer.

use std::ffi::CStr;
use std::mem::MaybeUninit;

use crate::color::pixel::kind::PixelFormatKind;
use crate::geo::Size;
use crate::{bind, EnumInt, Sdl};

use super::Renderer;

/// A kind of renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RendererKind {
    /// Software renderer, normally slow.
    Software,
    /// GPU accelerated renderer, normally fast.
    Accelerated,
}

/// An information of a renderer.
#[derive(Debug, Clone)]
pub struct RendererInfo {
    /// The name of the renderer.
    pub name: String,
    /// The kind of the renderer.
    pub kind: RendererKind,
    /// Whether vertical sync is enabled.
    pub is_v_sync: bool,
    /// Whether texture is supported.
    pub supported_texture: bool,
    /// The list of supported format kinds.
    pub supported_formats: Vec<PixelFormatKind>,
    /// The max size of texture.
    pub max_texture_size: Size,
}

impl From<bind::SDL_RendererInfo> for RendererInfo {
    #[allow(clippy::unnecessary_cast)]
    fn from(info: bind::SDL_RendererInfo) -> Self {
        let kind = if info.flags & bind::SDL_RENDERER_SOFTWARE as u32 == 0 {
            RendererKind::Accelerated
        } else {
            RendererKind::Software
        };
        let supported_formats = info
            .texture_formats
            .iter()
            .take(info.num_texture_formats as usize)
            .map(|&raw| raw as EnumInt)
            .map(PixelFormatKind::from_raw)
            .collect();
        Self {
            name: unsafe { CStr::from_ptr(info.name) }
                .to_str()
                .unwrap_or_default()
                .into(),
            kind,
            is_v_sync: info.flags & bind::SDL_RENDERER_PRESENTVSYNC as u32 != 0,
            supported_texture: info.flags & bind::SDL_RENDERER_TARGETTEXTURE as u32 != 0,
            supported_formats,
            max_texture_size: Size {
                width: info.max_texture_width as u32,
                height: info.max_texture_height as u32,
            },
        }
    }
}

/// An extension for [`Renderer`] to get [`RendererInfo`].
pub trait RendererInfoExt {
    /// Returns the information of the renderer.
    fn renderer_info(&self) -> RendererInfo;
}

impl<'window> RendererInfoExt for Renderer<'window> {
    fn renderer_info(&self) -> RendererInfo {
        let mut info_raw = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetRendererInfo(self.as_ptr(), info_raw.as_mut_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Getting renderer info");
        }
        unsafe { info_raw.assume_init() }.into()
    }
}
