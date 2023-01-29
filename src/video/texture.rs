//! Texture used to draw the image to [`Renderer`].

use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::color::Rgb;
use crate::geo::{Rect, Size};
use crate::renderer::Renderer;
use crate::surface::Surface;
use crate::{bind, EnumInt, Result, Sdl, SdlError};

pub mod lock;
mod query;

use lock::Lock;
pub use query::*;

/// An access type for the texture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
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
        match raw as EnumInt {
            bind::SDL_TEXTUREACCESS_STATIC => TextureAccess::Static,
            bind::SDL_TEXTUREACCESS_STREAMING => TextureAccess::Streaming,
            bind::SDL_TEXTUREACCESS_TARGET => TextureAccess::Target,
            _ => unreachable!(),
        }
    }

    #[allow(clippy::unnecessary_cast)]
    fn as_raw(&self) -> u32 {
        (match self {
            TextureAccess::Static => bind::SDL_TEXTUREACCESS_STATIC,
            TextureAccess::Streaming => bind::SDL_TEXTUREACCESS_STREAMING,
            TextureAccess::Target => bind::SDL_TEXTUREACCESS_TARGET,
        }) as u32
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
            .field("clip", &self.clip)
            .finish_non_exhaustive()
    }
}

assert_not_impl_all!(Texture: Send, Sync);

impl<'renderer> Texture<'renderer> {
    /// Constructs a texture from the renderer with access type.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to allocate, no rendering context was active, the format was unsupported, or the width or height were out of range.
    pub fn new(renderer: &'renderer Renderer<'renderer>, access: TextureAccess) -> Result<Self> {
        use super::window::ConfigExt;
        let Size { width, height } = renderer.window().size();
        let pixel_format = renderer.window().pixel_format();
        NonNull::new(unsafe {
            bind::SDL_CreateTexture(
                renderer.as_ptr(),
                pixel_format.as_raw(),
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

    /// Constructs a texture from the [`Surface`]. The texture will be readonly and the access type will be [`TextureAccess::Static`].
    pub fn from_surface(
        renderer: &'renderer Renderer<'renderer>,
        surface: &'renderer impl Surface,
    ) -> Self {
        let ptr = unsafe {
            bind::SDL_CreateTextureFromSurface(renderer.as_ptr(), surface.as_ptr().as_ptr())
        };
        Self {
            texture: NonNull::new(ptr).unwrap(),
            clip: None,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut bind::SDL_Texture {
        self.texture.as_ptr()
    }

    /// Returns the alpha mod of the texture.
    #[must_use]
    pub fn alpha_mod(&self) -> u8 {
        let mut alpha = 0;
        let ret = unsafe { bind::SDL_GetTextureAlphaMod(self.as_ptr(), &mut alpha) };
        if ret != 0 {
            Sdl::error_then_panic("Getting texture alpha mod");
        }
        alpha
    }

    /// Sets the alpha mod of the texture.
    ///
    /// # Errors
    ///
    /// Returns `Err` if setting the alpha mod is unsupported.
    ///
    /// # Panics
    ///
    /// Panics if some unrecoverable error is occurred.
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
        let ret = unsafe { bind::SDL_GetTextureColorMod(self.as_ptr(), &mut r, &mut g, &mut b) };
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
    #[must_use]
    pub fn clip(&self) -> &Option<Rect> {
        &self.clip
    }

    /// Sets the clip area.
    pub fn set_clip(&mut self, clip: Option<Rect>) {
        self.clip = clip;
    }

    /// Binds the texture to the current OpenGL context. And returns the size in the context.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to bind the texture.
    pub fn bind_to_current_gl_context(&self) -> Result<(f32, f32)> {
        let mut width = 0f32;
        let mut height = 0f32;
        let ret = unsafe { bind::SDL_GL_BindTexture(self.as_ptr(), &mut width, &mut height) };
        if ret == 0 {
            Ok((width, height))
        } else {
            Err(SdlError::Others { msg: Sdl::error() })
        }
    }

    /// Unbinds the texture to the current OpenGL context.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to unbind the texture.
    pub fn unbind_from_current_gl_context(&self) -> Result<()> {
        let ret = unsafe { bind::SDL_GL_UnbindTexture(self.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(SdlError::UnsupportedFeature)
        }
    }
}

impl Drop for Texture<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyTexture(self.as_ptr()) }
    }
}
