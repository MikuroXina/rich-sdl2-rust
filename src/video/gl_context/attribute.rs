//! Configuration of an OpenGL context.

use bitflags::bitflags;
use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;

use crate::{bind, Result, Sdl, SdlError};

use super::GlContext;

bitflags! {
    /// An attribute for OpenGL.
    pub struct GlAttributeKind: u32 {
        /// The minimum bits of the red channel in a color buffer.
        const RED_SIZE = bind::SDL_GLattr_SDL_GL_RED_SIZE;
        /// The minimum bits of the gree channel in a color buffer.
        const GREEN_SIZE = bind::SDL_GLattr_SDL_GL_GREEN_SIZE;
        /// The minimum bits of the blue channel in a color buffer.
        const BLUE_SIZE = bind::SDL_GLattr_SDL_GL_BLUE_SIZE;
        /// The minimum bits of the alpha channel in a color buffer.
        const ALPHA_SIZE = bind::SDL_GLattr_SDL_GL_ALPHA_SIZE;
        /// The minimum bits of the frame buffer.
        const BUFFER_SIZE = bind::SDL_GLattr_SDL_GL_BUFFER_SIZE;
        /// Whether the output is double buffered.
        const DOUBLEBUFFER = bind::SDL_GLattr_SDL_GL_DOUBLEBUFFER;
        /// The bits of the depth buffer.
        const DEPTH_SIZE = bind::SDL_GLattr_SDL_GL_DEPTH_SIZE;
        /// The bits of the stencil buffer.
        const STENCIL_SIZE = bind::SDL_GLattr_SDL_GL_STENCIL_SIZE;
        /// The minimum bits of the red channel in an accumulation buffer.
        const ACCUM_RED_SIZE = bind::SDL_GLattr_SDL_GL_ACCUM_RED_SIZE;
        /// The minimum bits of the gree channel in an accumulation buffer.
        const ACCUM_GREEN_SIZE = bind::SDL_GLattr_SDL_GL_ACCUM_GREEN_SIZE;
        /// The minimum bits of the blue channel in an accumulation buffer.
        const ACCUM_BLUE_SIZE = bind::SDL_GLattr_SDL_GL_ACCUM_BLUE_SIZE;
        /// The minimum bits of the alpha channel in an accumulation buffer.
        const ACCUM_ALPHA_SIZE = bind::SDL_GLattr_SDL_GL_ACCUM_ALPHA_SIZE;
        /// Whether the output is stereo 3D.
        const STEREO = bind::SDL_GLattr_SDL_GL_STEREO;
        /// The number of buffers used for multi-sample anti-aliasing.
        const MULTISAMPLEBUFFERS = bind::SDL_GLattr_SDL_GL_MULTISAMPLEBUFFERS;
        /// The number of samples used for multi-sample anti-aliasing.
        const MULTISAMPLESAMPLES = bind::SDL_GLattr_SDL_GL_MULTISAMPLESAMPLES;
        /// Whether the renderer uses hardware acceleration or force software rendering.
        const ACCELERATED_VISUAL = bind::SDL_GLattr_SDL_GL_ACCELERATED_VISUAL;
        /// OpenGL context major version.
        const CONTEXT_MAJOR_VERSION = bind::SDL_GLattr_SDL_GL_CONTEXT_MAJOR_VERSION;
        /// OpenGL context minor version.
        const CONTEXT_MINOR_VERSION = bind::SDL_GLattr_SDL_GL_CONTEXT_MINOR_VERSION;
        /// The value of a [`super::GlContextFlag`].
        const CONTEXT_FLAGS = bind::SDL_GLattr_SDL_GL_CONTEXT_FLAGS;
        /// The type of OpenGL context such as Core, Compatibility, ES, or etc.
        const CONTEXT_PROFILE_MASK = bind::SDL_GLattr_SDL_GL_CONTEXT_PROFILE_MASK;
        /// OpenGL context sharing.
        const SHARE_WITH_CURRENT_CONTEXT = bind::SDL_GLattr_SDL_GL_SHARE_WITH_CURRENT_CONTEXT;
        /// Whether to request sRGB capable visual.
        const FRAMEBUFFER_SRGB_CAPABLE = bind::SDL_GLattr_SDL_GL_FRAMEBUFFER_SRGB_CAPABLE;
        /// Setting what does on releasing OpenGL context. Can be set 0 (none) or 1 (flush).
        const CONTEXT_RELEASE_BEHAVIOR = bind::SDL_GLattr_SDL_GL_CONTEXT_RELEASE_BEHAVIOR;
        /// Whether to notify on reset the OpenGL context.
        const CONTEXT_RESET_NOTIFICATION = bind::SDL_GLattr_SDL_GL_CONTEXT_RESET_NOTIFICATION;
        /// Whether to disable all errors from OpenGL implementation. It might increase the performance and decrease power usage. But some inconsistency occurs the undefined behavior.
        const CONTEXT_NO_ERROR = bind::SDL_GLattr_SDL_GL_CONTEXT_NO_ERROR;
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
            .finish()
    }
}

assert_not_impl_all!(GlAttribute: Send, Sync);

impl<'gl> GlAttribute<'gl> {
    /// Constructs an attribute from context and kind.
    pub fn new(_: &'gl GlContext<'gl>, attr: GlAttributeKind) -> Self {
        Self {
            attr,
            _phantom: PhantomData,
        }
    }

    /// Sets the attribute value, or `Err` on failure.
    pub fn set(&self, value: i32) -> Result<()> {
        let ret = unsafe { bind::SDL_GL_SetAttribute(self.attr.bits, value) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(())
    }

    /// Gets the attribute value, or `Err` on failure.
    pub fn get(&self) -> Result<i32> {
        let mut value = 0;
        let ret = unsafe { bind::SDL_GL_GetAttribute(self.attr.bits, &mut value as *mut _) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(value)
    }
}
