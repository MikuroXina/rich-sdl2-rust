use std::marker::PhantomData;

use app::QuitEvent;

use crate::{bind, Sdl, Video};

pub mod app;

pub struct EventBox<'video> {
    quit_event_handlers: Vec<Box<dyn Fn(app::QuitEvent)>>,
    _phantom: PhantomData<&'video ()>,
}

impl<'video> EventBox<'video> {
    pub fn new(_: &'video Video) -> Self {
        let ret = unsafe { bind::SDL_InitSubSystem(bind::SDL_INIT_EVENTS) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl event")
        }
        Self {
            quit_event_handlers: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn handle_quit(&mut self, handler: Box<dyn Fn(app::QuitEvent)>) {
        self.quit_event_handlers.push(handler);
    }

    fn handle_event(&self, event: bind::SDL_Event) {
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

    pub fn poll(&self) {
        use std::mem::MaybeUninit;
        let mut event = MaybeUninit::uninit();
        let remaining_events = unsafe { bind::SDL_PollEvent(event.as_mut_ptr()) };
        let event = unsafe { event.assume_init() };
        if remaining_events == 0 {
            return;
        }
        self.handle_event(event);
    }

    pub fn wait_next_event_with(&self, timeout_ms: u32) {
        use std::mem::MaybeUninit;
        let mut event = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_WaitEventTimeout(event.as_mut_ptr(), timeout_ms as i32) };
        let event = unsafe { event.assume_init() };
        if ret == 0 {
            return;
        }
        self.handle_event(event);
    }
}

impl<'video> Drop for EventBox<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_EVENTS) }
    }
}
