//! Configuration of an OpenGL context.

#![allow(clippy::unnecessary_cast)]

use bitflags::bitflags;
use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;

use crate::{bind, EnumInt, Result, Sdl, SdlError};

use super::GlContext;

bitflags! {
    #[allow(clippy::unnecessary_cast)]
    /// An attribute for OpenGL.
    pub struct GlAttributeKind: u32 {
        /// The minimum bits of the red channel in a color buffer.
        const RED_SIZE = bind::SDL_GL_RED_SIZE as u32;
        /// The minimum bits of the gree channel in a color buffer.
        const GREEN_SIZE = bind::SDL_GL_GREEN_SIZE as u32;
        /// The minimum bits of the blue channel in a color buffer.
        const BLUE_SIZE = bind::SDL_GL_BLUE_SIZE as u32;
        /// The minimum bits of the alpha channel in a color buffer.
        const ALPHA_SIZE = bind::SDL_GL_ALPHA_SIZE as u32;
        /// The minimum bits of the frame buffer.
        const BUFFER_SIZE = bind::SDL_GL_BUFFER_SIZE as u32;
        /// Whether the output is double buffered.
        const DOUBLEBUFFER = bind::SDL_GL_DOUBLEBUFFER as u32;
        /// The bits of the depth buffer.
        const DEPTH_SIZE = bind::SDL_GL_DEPTH_SIZE as u32;
        /// The bits of the stencil buffer.
        const STENCIL_SIZE = bind::SDL_GL_STENCIL_SIZE as u32;
        /// The minimum bits of the red channel in an accumulation buffer.
        const ACCUM_RED_SIZE = bind::SDL_GL_ACCUM_RED_SIZE as u32;
        /// The minimum bits of the gree channel in an accumulation buffer.
        const ACCUM_GREEN_SIZE = bind::SDL_GL_ACCUM_GREEN_SIZE as u32;
        /// The minimum bits of the blue channel in an accumulation buffer.
        const ACCUM_BLUE_SIZE = bind::SDL_GL_ACCUM_BLUE_SIZE as u32;
        /// The minimum bits of the alpha channel in an accumulation buffer.
        const ACCUM_ALPHA_SIZE = bind::SDL_GL_ACCUM_ALPHA_SIZE as u32;
        /// Whether the output is stereo 3D.
        const STEREO = bind::SDL_GL_STEREO as u32;
        /// The number of buffers used for multi-sample anti-aliasing.
        const MULTISAMPLEBUFFERS = bind::SDL_GL_MULTISAMPLEBUFFERS as u32;
        /// The number of samples used for multi-sample anti-aliasing.
        const MULTISAMPLESAMPLES = bind::SDL_GL_MULTISAMPLESAMPLES as u32;
        /// Whether the renderer uses hardware acceleration or force software rendering.
        const ACCELERATED_VISUAL = bind::SDL_GL_ACCELERATED_VISUAL as u32;
        /// OpenGL context major version.
        const CONTEXT_MAJOR_VERSION = bind::SDL_GL_CONTEXT_MAJOR_VERSION as u32;
        /// OpenGL context minor version.
        const CONTEXT_MINOR_VERSION = bind::SDL_GL_CONTEXT_MINOR_VERSION as u32;
        /// The value of a [`super::GlContextFlag`].
        const CONTEXT_FLAGS = bind::SDL_GL_CONTEXT_FLAGS as u32;
        /// The type of OpenGL context such as Core, Compatibility, ES, or etc.
        const CONTEXT_PROFILE_MASK = bind::SDL_GL_CONTEXT_PROFILE_MASK as u32;
        /// OpenGL context sharing.
        const SHARE_WITH_CURRENT_CONTEXT = bind::SDL_GL_SHARE_WITH_CURRENT_CONTEXT as u32;
        /// Whether to request sRGB capable visual.
        const FRAMEBUFFER_SRGB_CAPABLE = bind::SDL_GL_FRAMEBUFFER_SRGB_CAPABLE as u32;
        /// Setting what does on releasing OpenGL context. Can be set 0 (none) or 1 (flush).
        const CONTEXT_RELEASE_BEHAVIOR = bind::SDL_GL_CONTEXT_RELEASE_BEHAVIOR as u32;
        /// Whether to notify on reset the OpenGL context.
        const CONTEXT_RESET_NOTIFICATION = bind::SDL_GL_CONTEXT_RESET_NOTIFICATION as u32;
        /// Whether to disable all errors from OpenGL implementation. It might increase the performance and decrease power usage. But some inconsistency occurs the undefined behavior.
        const CONTEXT_NO_ERROR = bind::SDL_GL_CONTEXT_NO_ERROR as u32;
    }
}

/// An attrubte of the OpenGL context.
pub struct GlAttribute<'gl> {
    attr: GlAttributeKind,
    _phantom: PhantomData<&'gl GlContext<'gl>>,
}

impl std::fmt::Debug for GlAttribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlAttribute")
            .field("attr", &self.attr)
            .finish_non_exhaustive()
    }
}

assert_not_impl_all!(GlAttribute: Send, Sync);

impl<'gl> GlAttribute<'gl> {
    /// Constructs an attribute from context and kind.
    #[must_use]
    pub fn new(_: &'gl GlContext<'gl>, attr: GlAttributeKind) -> Self {
        Self {
            attr,
            _phantom: PhantomData,
        }
    }

    /// Sets an attribute value.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to set a value for the attribute.
    pub fn set(&self, value: i32) -> Result<()> {
        let ret = unsafe { bind::SDL_GL_SetAttribute(self.attr.bits as EnumInt, value) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(())
    }

    /// Gets an attribute value.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to get a value of the attribute.
    pub fn get(&self) -> Result<i32> {
        let mut value = 0;
        let ret = unsafe { bind::SDL_GL_GetAttribute(self.attr.bits as EnumInt, &mut value) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(value)
    }
}
