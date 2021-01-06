pub fn main() {
    use sdl2_rust::window::WindowPos;
    use sdl2_rust::*;

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
        let pen = renderer::pen::Pen::new(&renderer);
        pen.set_color(renderer::pen::Color {
            r: 127,
            g: 127,
            b: 0,
        });
        pen.clear();

        event.poll();
    }
}
