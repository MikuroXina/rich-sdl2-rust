use std::ffi::CStr;
use std::mem::MaybeUninit;

use crate::geo::Size;
use crate::pixel_format::kind::PixelFormatKind;
use crate::{bind, Sdl};

use super::Renderer;

pub enum RendererKind {
    Software,
    Accelerated,
}

pub struct RendererInfo {
    pub name: String,
    pub kind: RendererKind,
    pub is_v_sync: bool,
    pub supported_texture: bool,
    pub supported_formats: Vec<PixelFormatKind>,
    pub max_texture_size: Size,
}

impl From<bind::SDL_RendererInfo> for RendererInfo {
    fn from(info: bind::SDL_RendererInfo) -> Self {
        let kind = if info.flags & bind::SDL_RendererFlags_SDL_RENDERER_SOFTWARE != 0 {
            RendererKind::Software
        } else {
            RendererKind::Accelerated
        };
        let supported_formats = info
            .texture_formats
            .iter()
            .take(info.num_texture_formats as usize)
            .map(|&format| format.into())
            .collect();
        Self {
            name: unsafe { CStr::from_ptr(info.name) }
                .to_str()
                .unwrap_or_default()
                .into(),
            kind,
            is_v_sync: info.flags & bind::SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC != 0,
            supported_texture: info.flags & bind::SDL_RendererFlags_SDL_RENDERER_TARGETTEXTURE != 0,
            supported_formats,
            max_texture_size: Size {
                width: info.max_texture_width as u32,
                height: info.max_texture_height as u32,
            },
        }
    }
}

pub trait RendererInfoExt {
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
