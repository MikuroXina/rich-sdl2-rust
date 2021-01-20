use sdl2_rust::{
    color::Rgb,
    renderer::pen::Pen,
    window::{self, WindowPos},
    *,
};

pub fn main() {
    let sdl = Sdl::new();
    let video = Video::new(&sdl);
    let window = window::WindowBuilder::default().build(&video);
    let renderer = renderer::Renderer::new(&window);

    use std::cell::Cell;
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
