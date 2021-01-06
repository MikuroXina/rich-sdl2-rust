use crate::{bind, Sdl};

use super::Renderer;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Pen<'renderer> {
    renderer: &'renderer Renderer<'renderer>,
}

impl<'renderer> Pen<'renderer> {
    pub fn new(renderer: &'renderer Renderer) -> Self {
        Self { renderer }
    }

    pub fn set_color(&self, Color { r, g, b }: Color) {
        let ret = unsafe { bind::SDL_SetRenderDrawColor(self.renderer.as_ptr(), r, g, b, 255) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen color")
        }
    }

    pub fn clear(&self) {
        let ret = unsafe { bind::SDL_RenderClear(self.renderer.as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen clear")
        }
    }
}
