use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;
use std::os::raw::c_int;

use crate::window::Window;
use crate::{bind, Sdl};

pub struct Cursor<'window> {
    window: PhantomData<&'window Window<'window>>,
}

impl std::fmt::Debug for Cursor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cursor").finish()
    }
}

assert_not_impl_all!(Cursor: Send, Sync);

impl<'window> Cursor<'window> {
    pub fn new(_: &'window Window) -> Self {
        Self {
            window: PhantomData,
        }
    }

    pub fn show(&self) {
        let ret = unsafe { bind::SDL_ShowCursor(bind::SDL_ENABLE as c_int) };
        if ret < 0 {
            eprintln!("{}", Sdl::error());
        }
    }

    pub fn hide(&self) {
        let ret = unsafe { bind::SDL_ShowCursor(bind::SDL_DISABLE as c_int) };
        if ret < 0 {
            eprintln!("{}", Sdl::error());
        }
    }

    pub fn is_shown(&self) -> bool {
        let ret = unsafe { bind::SDL_ShowCursor(bind::SDL_QUERY as c_int) };
        if ret < 0 {
            eprintln!("{}", Sdl::error());
            return true;
        }
        ret as u32 == bind::SDL_ENABLE
    }
}
