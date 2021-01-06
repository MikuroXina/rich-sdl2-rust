use std::marker::PhantomData;
use std::ptr::NonNull;

use super::Window;
use crate::{bind, Sdl, Video};

#[derive(Debug)]
pub enum WindowPos {
    Coord(u32),
    Undefined,
    Centered,
}

impl WindowPos {
    fn into_arg(self) -> std::os::raw::c_int {
        use WindowPos::*;
        (match self {
            Coord(coord) => coord,
            Undefined => 0x1FFF0000, // SDL_WINDOWPOS_UNDEFINED
            Centered => 0x2FFF0000,  // SDL_WINDOWPOS_CENTERED
        }) as std::os::raw::c_int
    }
}

#[derive(Debug)]
pub struct WindowBuilder {
    title: String,
    x: WindowPos,
    y: WindowPos,
    width: u32,
    height: u32,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            title: "Untitled".into(),
            x: WindowPos::Centered,
            y: WindowPos::Centered,
            width: 640,
            height: 480,
        }
    }
}

impl WindowBuilder {
    pub fn title(&mut self, title: impl ToOwned<Owned = String>) -> &mut Self {
        self.title = title.to_owned();
        self
    }

    pub fn x(&mut self, x: WindowPos) -> &mut Self {
        self.x = x;
        self
    }

    pub fn y(&mut self, y: WindowPos) -> &mut Self {
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
                self.x.into_arg(),
                self.y.into_arg(),
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
