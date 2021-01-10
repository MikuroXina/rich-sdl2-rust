use crate::{bind, Result, SdlError};

use super::GlContext;

pub enum IntervalKind {
    AdaptiveVerticalSync,
    Immediate,
    VerticalSync,
}

pub trait BufferExt {
    fn set_swap_interval(&self, interval_kind: IntervalKind) -> Result<()>;
    fn swap_buffer(&self);
}

impl BufferExt for GlContext<'_> {
    fn set_swap_interval(&self, interval_kind: IntervalKind) -> Result<()> {
        let ret = unsafe {
            bind::SDL_GL_SetSwapInterval(match interval_kind {
                IntervalKind::AdaptiveVerticalSync => -1,
                IntervalKind::Immediate => 0,
                IntervalKind::VerticalSync => 1,
            })
        };
        if ret != 0 {
            return Err(SdlError::UnsupportedFeature);
        }
        Ok(())
    }

    fn swap_buffer(&self) {
        unsafe { bind::SDL_GL_SwapWindow(self.window.as_ptr()) }
    }
}
