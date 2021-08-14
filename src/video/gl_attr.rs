use crate::bind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlAttribute {
    RedSize,
    GreenSize,
    BlueSize,
    AlphaSize,
    BufferSize,
    Doublebuffer,
    DepthSize,
    StencilSize,
    AccumRedSize,
    AccumGreenSize,
    AccumBlueSize,
    AccumAlphaSize,
    Stereo,
    Multisamplebuffers,
    Multisamplesamples,
    AcceleratedVisual,
    RetainedBacking,
    ContextMajorVersion,
    ContextMinorVersion,
    ContextEgl,
    ContextFlags,
    ContextProfileMask,
    ShareWithCurrentContext,
    FramebufferSrgbCapable,
    ContextReleaseBehavior,
    ContextResetNotification,
    ContextNoError,
}

impl From<GlAttribute> for bind::SDL_GLattr {
    fn from(attr: GlAttribute) -> Self {
        use GlAttribute::*;
        match attr {
            RedSize => bind::SDL_GLattr_SDL_GL_RED_SIZE,
            GreenSize => bind::SDL_GLattr_SDL_GL_GREEN_SIZE,
            BlueSize => bind::SDL_GLattr_SDL_GL_BLUE_SIZE,
            AlphaSize => bind::SDL_GLattr_SDL_GL_ALPHA_SIZE,
            BufferSize => bind::SDL_GLattr_SDL_GL_BUFFER_SIZE,
            Doublebuffer => bind::SDL_GLattr_SDL_GL_DOUBLEBUFFER,
            DepthSize => bind::SDL_GLattr_SDL_GL_DEPTH_SIZE,
            StencilSize => bind::SDL_GLattr_SDL_GL_STENCIL_SIZE,
            AccumRedSize => bind::SDL_GLattr_SDL_GL_ACCUM_RED_SIZE,
            AccumGreenSize => bind::SDL_GLattr_SDL_GL_ACCUM_GREEN_SIZE,
            AccumBlueSize => bind::SDL_GLattr_SDL_GL_ACCUM_BLUE_SIZE,
            AccumAlphaSize => bind::SDL_GLattr_SDL_GL_ACCUM_ALPHA_SIZE,
            Stereo => bind::SDL_GLattr_SDL_GL_STEREO,
            Multisamplebuffers => bind::SDL_GLattr_SDL_GL_MULTISAMPLEBUFFERS,
            Multisamplesamples => bind::SDL_GLattr_SDL_GL_MULTISAMPLESAMPLES,
            AcceleratedVisual => bind::SDL_GLattr_SDL_GL_ACCELERATED_VISUAL,
            RetainedBacking => bind::SDL_GLattr_SDL_GL_RETAINED_BACKING,
            ContextMajorVersion => bind::SDL_GLattr_SDL_GL_CONTEXT_MAJOR_VERSION,
            ContextMinorVersion => bind::SDL_GLattr_SDL_GL_CONTEXT_MINOR_VERSION,
            ContextEgl => bind::SDL_GLattr_SDL_GL_CONTEXT_EGL,
            ContextFlags => bind::SDL_GLattr_SDL_GL_CONTEXT_FLAGS,
            ContextProfileMask => bind::SDL_GLattr_SDL_GL_CONTEXT_PROFILE_MASK,
            ShareWithCurrentContext => bind::SDL_GLattr_SDL_GL_SHARE_WITH_CURRENT_CONTEXT,
            FramebufferSrgbCapable => bind::SDL_GLattr_SDL_GL_FRAMEBUFFER_SRGB_CAPABLE,
            ContextReleaseBehavior => bind::SDL_GLattr_SDL_GL_CONTEXT_RELEASE_BEHAVIOR,
            ContextResetNotification => bind::SDL_GLattr_SDL_GL_CONTEXT_RESET_NOTIFICATION,
            ContextNoError => bind::SDL_GLattr_SDL_GL_CONTEXT_NO_ERROR,
        }
    }
}

impl From<bind::SDL_GLattr> for GlAttribute {
    fn from(attr: bind::SDL_GLattr) -> Self {
        use GlAttribute::*;
        match attr {
            bind::SDL_GLattr_SDL_GL_RED_SIZE => RedSize,
            bind::SDL_GLattr_SDL_GL_GREEN_SIZE => GreenSize,
            bind::SDL_GLattr_SDL_GL_BLUE_SIZE => BlueSize,
            bind::SDL_GLattr_SDL_GL_ALPHA_SIZE => AlphaSize,
            bind::SDL_GLattr_SDL_GL_BUFFER_SIZE => BufferSize,
            bind::SDL_GLattr_SDL_GL_DOUBLEBUFFER => Doublebuffer,
            bind::SDL_GLattr_SDL_GL_DEPTH_SIZE => DepthSize,
            bind::SDL_GLattr_SDL_GL_STENCIL_SIZE => StencilSize,
            bind::SDL_GLattr_SDL_GL_ACCUM_RED_SIZE => AccumRedSize,
            bind::SDL_GLattr_SDL_GL_ACCUM_GREEN_SIZE => AccumGreenSize,
            bind::SDL_GLattr_SDL_GL_ACCUM_BLUE_SIZE => AccumBlueSize,
            bind::SDL_GLattr_SDL_GL_ACCUM_ALPHA_SIZE => AccumAlphaSize,
            bind::SDL_GLattr_SDL_GL_STEREO => Stereo,
            bind::SDL_GLattr_SDL_GL_MULTISAMPLEBUFFERS => Multisamplebuffers,
            bind::SDL_GLattr_SDL_GL_MULTISAMPLESAMPLES => Multisamplesamples,
            bind::SDL_GLattr_SDL_GL_ACCELERATED_VISUAL => AcceleratedVisual,
            bind::SDL_GLattr_SDL_GL_RETAINED_BACKING => RetainedBacking,
            bind::SDL_GLattr_SDL_GL_CONTEXT_MAJOR_VERSION => ContextMajorVersion,
            bind::SDL_GLattr_SDL_GL_CONTEXT_MINOR_VERSION => ContextMinorVersion,
            bind::SDL_GLattr_SDL_GL_CONTEXT_EGL => ContextEgl,
            bind::SDL_GLattr_SDL_GL_CONTEXT_FLAGS => ContextFlags,
            bind::SDL_GLattr_SDL_GL_CONTEXT_PROFILE_MASK => ContextProfileMask,
            bind::SDL_GLattr_SDL_GL_SHARE_WITH_CURRENT_CONTEXT => ShareWithCurrentContext,
            bind::SDL_GLattr_SDL_GL_FRAMEBUFFER_SRGB_CAPABLE => FramebufferSrgbCapable,
            bind::SDL_GLattr_SDL_GL_CONTEXT_RELEASE_BEHAVIOR => ContextReleaseBehavior,
            bind::SDL_GLattr_SDL_GL_CONTEXT_RESET_NOTIFICATION => ContextResetNotification,
            bind::SDL_GLattr_SDL_GL_CONTEXT_NO_ERROR => ContextNoError,
            _ => unreachable!(),
        }
    }
}
