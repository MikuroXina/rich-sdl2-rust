use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::{bind, Sdl, Video};

pub struct Window<'video> {
    window: NonNull<bind::SDL_Window>,
    _phantom: PhantomData<&'video ()>,
}

impl<'video> Window<'video> {}

impl<'video> Drop for Window<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DestroyWindow(self.window.as_ptr()) }
    }
}

#[derive(Default, Debug)]
pub struct WindowBuilder {
    title: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl WindowBuilder {
    pub fn title(&mut self, title: impl ToOwned<Owned = String>) -> &mut Self {
        self.title = title.to_owned();
        self
    }

    pub fn x(&mut self, x: u32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn y(&mut self, y: u32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn build<'video>(self, _: &'video Video) -> Window<'video> {
        use std::os::raw::{c_char, c_int};
        let raw = unsafe {
            bind::SDL_CreateWindow(
                self.title.as_ptr() as *const c_char,
                self.x as c_int,
                self.y as c_int,
                self.width as c_int,
                self.height as c_int,
                0,
            )
        };
        NonNull::new(raw).map_or_else(
            || {
                eprintln!("Sdl window error: {}", Sdl::poll_error());
                panic!("Sdl window initialization failed");
            },
            |window| Window {
                window,
                _phantom: PhantomData,
            },
        )
    }
}
