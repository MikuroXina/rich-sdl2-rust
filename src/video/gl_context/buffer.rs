use crate::{bind, Result, SdlError};

use super::GlContext;

/// A kind of the interval of swapping buffers in an OpenGL context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IntervalKind {
    /// Vertical syncing but swap immediately on failed.
    AdaptiveVerticalSync,
    /// Always swap immediately.
    Immediate,
    /// Vertical syncing.
    VerticalSync,
}

/// An extension for [`GlContext`] to add methods for control buffers.
pub trait BufferExt {
    /// Sets the interval mode of swapping buffers.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the feature swapping them is unsupported.
    fn set_swap_interval(&self, interval_kind: IntervalKind) -> Result<()>;

    /// Swaps buffers immediately.
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
