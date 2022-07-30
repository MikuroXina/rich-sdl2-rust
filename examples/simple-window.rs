use rich_sdl2_rust::{
    color::Rgb,
    renderer::pen::Pen,
    ttf::{
        font::{pen, Font},
        Ttf,
    },
    *,
};
use std::cell::{Cell, RefCell};

pub fn main() {
    let font_file = std::env::args()
        .skip(1)
        .next()
        .expect("please specify font file on arg");
    let sdl = Sdl::new();
    let ttf = Ttf::new();
    let font = Font::new(&ttf, &font_file, 20, None).expect("Arial font not found");
    let video = Video::new(&sdl);
    let window = window::WindowBuilder::builder().build().new_window(&video);
    let renderer = renderer::Renderer::new(&window);

    let message = RefCell::new(String::new());
    let exit = Cell::new(false);
    let mut event = EventBox::new(&video);
    event.handle_window(Box::new(|e| {
        *message.borrow_mut() = format!("window event: {:?} {:?}", e.timestamp, e.details);
    }));
    event.handle_quit(Box::new(|e| {
        eprintln!("quit event: {}", e.timestamp);
        exit.set(true);
    }));

    while !exit.get() {
        use pen::{FontRenderExt, FontRenderOptions};
        let pen = Pen::new(&renderer);
        pen.set_color(Rgb {
            r: 127,
            g: 127,
            b: 0,
        });
        pen.clear();
        pen.text(&font, &message.borrow(), FontRenderOptions::default());

        event.poll();
    }
}
