//! Texture used to draw the image to [`Renderer`].

use static_assertions::assert_not_impl_all;
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

/// An access type for the texture.
pub enum TextureAccess {
    /// Cannot mutate and lock.
    Static,
    /// Can mutate and lock.
    Streaming,
    /// Can use as the render target in [`Renderer`].
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

/// A texture used to draw the image to [`Renderer`].
pub struct Texture<'renderer> {
    texture: NonNull<bind::SDL_Texture>,
    clip: Option<Rect>,
    _phantom: PhantomData<&'renderer ()>,
}

impl std::fmt::Debug for Texture<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture")
            .field("texture", &self.texture)
            .field("clip", &self.clip)
            .finish()
    }
}

assert_not_impl_all!(Texture: Send, Sync);

impl<'renderer> Texture<'renderer> {
    /// Constructs a texture from the renderer with access type, or `Err` on failure.
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

    /// Returns the alpha mod of the texture.
    pub fn alpha_mod(&self) -> u8 {
        let mut alpha = 0;
        let ret = unsafe { bind::SDL_GetTextureAlphaMod(self.as_ptr(), &mut alpha as *mut _) };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture alpha mod");
        }
        alpha
    }

    /// Sets the alpha mod of the texture.
    pub fn set_alpha_mod(&self, alpha: u8) -> Result<()> {
        let ret = unsafe { bind::SDL_SetTextureAlphaMod(self.as_ptr(), alpha) };
        if ret != 0 {
            let error = Sdl::error();
            if error == "That operation is not supported" {
                return Err(SdlError::UnsupportedFeature);
            }
            Sdl::error_then_panic("Setting texture alpha mod");
        }
        Ok(())
    }

    /// Returns the color mod of the texture.
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

    /// Sets the color mod of the texture.
    pub fn set_color_mod(&self, Rgb { r, g, b }: Rgb) {
        let ret = unsafe { bind::SDL_SetTextureColorMod(self.as_ptr(), r, g, b) };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture color mod");
        }
    }

    /// Obtains the lock for the texture in area, or whole if `None`.
    pub fn lock(&'renderer mut self, area: Option<Rect>) -> Lock<'renderer> {
        Lock::new(self, area)
    }

    /// Return the clip area of the texture if available.
    pub fn clip(&self) -> &Option<Rect> {
        &self.clip
    }

    /// Sets the clip area.
    pub fn set_clip(&mut self, clip: Option<Rect>) {
        self.clip = clip;
    }

    /// Binds the texture to the current OpenGL context. And returns the size in the context, or `Err` on failure.
    pub fn bind_to_current_gl_context(&self) -> Result<(f32, f32)> {
        let mut width = 0f32;
        let mut height = 0f32;
        let ret = unsafe {
            bind::SDL_GL_BindTexture(self.as_ptr(), &mut width as *mut _, &mut height as *mut _)
        };
        if ret != 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok((width, height))
        }
    }

    /// Unbinds the texture to the current OpenGL context, or `Err` on failure.
    pub fn unbind_from_current_gl_context(&self) -> Result<()> {
        let ret = unsafe { bind::SDL_GL_UnbindTexture(self.as_ptr()) };
        if ret != 0 {
            Err(SdlError::UnsupportedFeature)
        } else {
            Ok(())
        }
    }
}

impl Drop for Texture<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyTexture(self.as_ptr()) }
    }
}
