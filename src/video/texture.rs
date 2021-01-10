use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::geo::Size;
use crate::renderer::Renderer;
use crate::{bind, Result, Sdl, SdlError};

pub enum TextureAccess {
    Static,
    Streaming,
    Target,
}

impl TextureAccess {
    fn as_raw(&self) -> u32 {
        match self {
            TextureAccess::Static => bind::SDL_TextureAccess_SDL_TEXTUREACCESS_STATIC,
            TextureAccess::Streaming => bind::SDL_TextureAccess_SDL_TEXTUREACCESS_STREAMING,
            TextureAccess::Target => bind::SDL_TextureAccess_SDL_TEXTUREACCESS_TARGET,
        }
    }
}

pub struct Texture<'renderer> {
    texture: NonNull<bind::SDL_Texture>,
    _phantom: PhantomData<&'renderer ()>,
}

impl<'renderer> Texture<'renderer> {
    pub fn new(renderer: &'renderer Renderer<'renderer>, access: TextureAccess) -> Result<Self> {
        use super::window::ConfigExt;
        let Size { width, height } = renderer.window().size();
        let pixel_format = renderer.window().pixel_format();
        NonNull::new(unsafe {
            bind::SDL_CreateTexture(
                renderer.as_ptr(),
                pixel_format.into(),
                access.as_raw() as i32,
                width as i32,
                height as i32,
            )
        })
        .map_or_else(
            || Err(SdlError::UnsupportedFeature),
            |texture| {
                Ok(Self {
                    texture,
                    _phantom: PhantomData,
                })
            },
        )
    }
}
