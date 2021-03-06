use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::color::Rgb;
use crate::geo::{Rect, Size};
use crate::renderer::Renderer;
use crate::{bind, Result, Sdl, SdlError};

pub mod lock;
mod query;

use lock::Lock;
pub use query::*;

pub enum TextureAccess {
    Static,
    Streaming,
    Target,
}

impl TextureAccess {
    fn from_raw(raw: u32) -> Self {
        match raw {
            bind::SDL_TextureAccess_SDL_TEXTUREACCESS_STATIC => TextureAccess::Static,
            bind::SDL_TextureAccess_SDL_TEXTUREACCESS_STREAMING => TextureAccess::Streaming,
            bind::SDL_TextureAccess_SDL_TEXTUREACCESS_TARGET => TextureAccess::Target,
            _ => unreachable!(),
        }
    }

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
    clip: Option<Rect>,
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
                    clip: None,
                    _phantom: PhantomData,
                })
            },
        )
    }

    pub(crate) fn as_ptr(&self) -> *mut bind::SDL_Texture {
        self.texture.as_ptr()
    }

    pub fn alpha_mod(&self) -> u8 {
        let mut alpha = 0;
        let ret = unsafe { bind::SDL_GetTextureAlphaMod(self.as_ptr(), &mut alpha as *mut _) };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture alpha mod");
        }
        alpha
    }

    pub fn set_alpha_mod(&self, alpha: u8) -> Result<()> {
        let ret = unsafe { bind::SDL_SetTextureAlphaMod(self.as_ptr(), alpha) };
        if ret != 0 {
            let error = Sdl::error();
            if error == "That operation is not supported" {
                return Err(SdlError::UnsupportedFeature);
            }
            eprintln!("Setting texture alpha mod error: {}", error);
            panic!("Unrecoverable Sdl error occurred")
        }
        Ok(())
    }

    pub fn color_mod(&self) -> Rgb {
        let (mut r, mut g, mut b) = (0, 0, 0);
        let ret = unsafe {
            bind::SDL_GetTextureColorMod(
                self.as_ptr(),
                &mut r as *mut _,
                &mut g as *mut _,
                &mut b as *mut _,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture color mod");
        }
        Rgb { r, g, b }
    }

    pub fn set_color_mod(&self, Rgb { r, g, b }: Rgb) {
        let ret = unsafe { bind::SDL_SetTextureColorMod(self.as_ptr(), r, g, b) };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture color mod");
        }
    }

    pub fn lock(&'renderer mut self, area: Option<Rect>) -> Lock<'renderer> {
        Lock::new(self, area)
    }

    pub fn clip(&self) -> &Option<Rect> {
        &self.clip
    }

    pub fn set_clip(&mut self, clip: Option<Rect>) {
        self.clip = clip;
    }
}

impl Drop for Texture<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyTexture(self.as_ptr()) }
    }
}
