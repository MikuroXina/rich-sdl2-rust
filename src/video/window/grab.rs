use crate::bind;

use super::Window;

pub trait MouseGrabExt {
    fn grab(&self);
    fn ungrab(&self);
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
