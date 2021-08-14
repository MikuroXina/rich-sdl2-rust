use bitflags::bitflags;
use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;

use crate::{bind, video::gl_attr, Result, Sdl, SdlError};

use super::GlContext;

bitflags! {
    pub struct AttributeKind: u32 {
        const RED_SIZE = bind::SDL_GLattr_SDL_GL_RED_SIZE;
        const GREEN_SIZE = bind::SDL_GLattr_SDL_GL_GREEN_SIZE;
        const BLUE_SIZE = bind::SDL_GLattr_SDL_GL_BLUE_SIZE;
        const ALPHA_SIZE = bind::SDL_GLattr_SDL_GL_ALPHA_SIZE;
        const BUFFER_SIZE = bind::SDL_GLattr_SDL_GL_BUFFER_SIZE;
        const DOUBLEBUFFER = bind::SDL_GLattr_SDL_GL_DOUBLEBUFFER;
        const DEPTH_SIZE = bind::SDL_GLattr_SDL_GL_DEPTH_SIZE;
        const STENCIL_SIZE = bind::SDL_GLattr_SDL_GL_STENCIL_SIZE;
        const ACCUM_RED_SIZE = bind::SDL_GLattr_SDL_GL_ACCUM_RED_SIZE;
        const ACCUM_GREEN_SIZE = bind::SDL_GLattr_SDL_GL_ACCUM_GREEN_SIZE;
        const ACCUM_BLUE_SIZE = bind::SDL_GLattr_SDL_GL_ACCUM_BLUE_SIZE;
        const ACCUM_ALPHA_SIZE = bind::SDL_GLattr_SDL_GL_ACCUM_ALPHA_SIZE;
        const STEREO = bind::SDL_GLattr_SDL_GL_STEREO;
        const MULTISAMPLEBUFFERS = bind::SDL_GLattr_SDL_GL_MULTISAMPLEBUFFERS;
        const MULTISAMPLESAMPLES = bind::SDL_GLattr_SDL_GL_MULTISAMPLESAMPLES;
        const ACCELERATED_VISUAL = bind::SDL_GLattr_SDL_GL_ACCELERATED_VISUAL;
        const RETAINED_BACKING = bind::SDL_GLattr_SDL_GL_RETAINED_BACKING;
        const CONTEXT_MAJOR_VERSION = bind::SDL_GLattr_SDL_GL_CONTEXT_MAJOR_VERSION;
        const CONTEXT_MINOR_VERSION = bind::SDL_GLattr_SDL_GL_CONTEXT_MINOR_VERSION;
        const CONTEXT_EGL = bind::SDL_GLattr_SDL_GL_CONTEXT_EGL;
        const CONTEXT_FLAGS = bind::SDL_GLattr_SDL_GL_CONTEXT_FLAGS;
        const CONTEXT_PROFILE_MASK = bind::SDL_GLattr_SDL_GL_CONTEXT_PROFILE_MASK;
        const SHARE_WITH_CURRENT_CONTEXT = bind::SDL_GLattr_SDL_GL_SHARE_WITH_CURRENT_CONTEXT;
        const FRAMEBUFFER_SRGB_CAPABLE = bind::SDL_GLattr_SDL_GL_FRAMEBUFFER_SRGB_CAPABLE;
        const CONTEXT_RELEASE_BEHAVIOR = bind::SDL_GLattr_SDL_GL_CONTEXT_RELEASE_BEHAVIOR;
        const CONTEXT_RESET_NOTIFICATION = bind::SDL_GLattr_SDL_GL_CONTEXT_RESET_NOTIFICATION;
        const CONTEXT_NO_ERROR = bind::SDL_GLattr_SDL_GL_CONTEXT_NO_ERROR;
    }
}

pub struct GlAttribute<'gl> {
    attr: bind::SDL_GLattr,
    _phantom: PhantomData<&'gl GlContext<'gl>>,
}

impl std::fmt::Debug for GlAttribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attr: gl_attr::GlAttribute = self.attr.into();
        f.debug_struct("GlAttribute").field("attr", &attr).finish()
    }
}

assert_not_impl_all!(GlAttribute: Send, Sync);

impl<'gl> GlAttribute<'gl> {
    pub fn new(_: &'gl GlContext<'gl>, kind: AttributeKind) -> Self {
        Self {
            attr: kind.bits,
            _phantom: PhantomData,
        }
    }

    pub fn set(&self, value: i32) -> Result<()> {
        let ret = unsafe { bind::SDL_GL_SetAttribute(self.attr, value) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(())
    }

    pub fn get(&self) -> Result<i32> {
        let mut value = 0;
        let ret = unsafe { bind::SDL_GL_GetAttribute(self.attr, &mut value as *mut _) };
        if ret != 0 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Ok(value)
    }
}
