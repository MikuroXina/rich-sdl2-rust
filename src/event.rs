use std::marker::PhantomData;

use app::QuitEvent;

use crate::{bind, Video};

pub mod app;

pub struct EventBox<'video> {
    quit_event_handlers: Vec<Box<dyn Fn(app::QuitEvent)>>,
    _phantom: PhantomData<&'video ()>,
}

impl<'video> EventBox<'video> {
    pub fn new(_: &'video Video) -> Self {
        Self {
            quit_event_handlers: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn handle_quit(&mut self, handler: Box<dyn Fn(app::QuitEvent)>) {
        self.quit_event_handlers.push(handler);
    }

    pub fn poll(&self) {
        use std::mem::MaybeUninit;
        let mut event = MaybeUninit::uninit();
        let remaining_events = unsafe { bind::SDL_PollEvent(event.as_mut_ptr()) };
        let event = unsafe { event.assume_init() };
        if remaining_events == 0 {
            return;
        }
        let ty = unsafe { event.type_ };
        eprintln!("event type: {}", ty);
        match ty {
            bind::SDL_EventType_SDL_QUIT => {
                let quit: QuitEvent = unsafe { event.quit }.into();
                self.quit_event_handlers
                    .iter()
                    .for_each(|handler| handler(quit.clone()))
            }
            _ => {}
        }
    }
}
