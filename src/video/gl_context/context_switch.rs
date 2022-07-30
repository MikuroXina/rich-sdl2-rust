use crate::window::{Window, WindowContextKind};
use crate::{bind, Result, SdlError};

use super::GlContext;

/// An extension for [`Window`] to set OpenGL context.
pub trait ContextSwitchExt<'window> {
    /// Sets context to the window.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to set the context to.
    fn set_context<'context: 'window>(&'window self, context: GlContext<'context>) -> Result<()>;
}

impl<'window> ContextSwitchExt<'window> for Window<'window> {
    fn set_context<'context: 'window>(&'window self, context: GlContext<'context>) -> Result<()> {
        if let WindowContextKind::OpenGl = self.state().context_kind {
            return Err(SdlError::Others {
                msg: "Invalid context".into(),
            });
        }
        let ret = unsafe { bind::SDL_GL_MakeCurrent(self.as_ptr(), context.as_ptr()) };
        if ret != 0 {
            return Err(SdlError::UnsupportedFeature);
        }
        Ok(())
    }
}
