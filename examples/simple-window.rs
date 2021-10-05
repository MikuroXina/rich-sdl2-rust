use rich_sdl2_rust::{color::Rgb, renderer::pen::Pen, *};
use std::cell::Cell;

pub fn main() {
    let sdl = Sdl::new();
    let video = Video::new(&sdl);
    let window = window::WindowBuilder::builder().build().new_window(&video);
    let renderer = renderer::Renderer::new(&window);

    let exit = Cell::new(false);
    let mut event = EventBox::new(&video);
    event.handle_window(Box::new(|e| {
        eprintln!("window event: {:?}", e);
    }));
    event.handle_quit(Box::new(|e| {
        eprintln!("quit event: {}", e.timestamp);
        exit.set(true);
    }));

    while !exit.get() {
        let pen = Pen::new(&renderer);
        pen.set_color(Rgb {
            r: 127,
            g: 127,
            b: 0,
        });
        pen.clear();

        event.poll();
    }
}
