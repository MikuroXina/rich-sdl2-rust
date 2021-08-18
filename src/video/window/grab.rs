use crate::bind;

use super::Window;

/// An extension for [`Window`] to grab/ungrab the mouse.
pub trait MouseGrabExt {
    /// Grabs the mouse.
    fn grab(&self);
    /// Ungrabs the mouse.
    fn ungrab(&self);
    /// Returns whether the window is grabbing the mouse.
    fn is_grabbed(&self) -> bool;
}

impl MouseGrabExt for Window<'_> {
    fn grab(&self) {
        unsafe { bind::SDL_SetWindowGrab(self.as_ptr(), 1) }
    }

    fn ungrab(&self) {
        unsafe { bind::SDL_SetWindowGrab(self.as_ptr(), 0) }
    }

    fn is_grabbed(&self) -> bool {
        unsafe { bind::SDL_GetWindowGrab(self.as_ptr()) != 0 }
    }
}
