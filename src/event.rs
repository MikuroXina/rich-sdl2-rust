use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;

use self::{
    app::QuitEvent,
    keyboard::KeyboardEvent,
    mouse::{MouseButtonEvent, MouseEvent, MouseMotionEvent, MouseWheelEvent},
    text::{TextEditingEvent, TextInputEvent},
    window::WindowEvent,
};

use crate::{bind, Sdl, Video};

pub mod app;
pub mod game_controller;
pub mod joystick;
pub mod keyboard;
pub mod mouse;
pub mod sensor;
pub mod text;
pub mod window;

pub type EventHandler<'video, T> = Box<dyn Fn(T) + 'video>;

pub struct EventBox<'video> {
    quit_event_handlers: Vec<EventHandler<'video, QuitEvent>>,
    window_event_handlers: Vec<EventHandler<'video, WindowEvent>>,
    keyboard_event_handlers: Vec<EventHandler<'video, KeyboardEvent>>,
    input_event_handlers: Vec<EventHandler<'video, TextInputEvent>>,
    editing_event_handlers: Vec<EventHandler<'video, TextEditingEvent>>,
    mouse_event_handlers: Vec<EventHandler<'video, MouseEvent>>,
    _phantom: PhantomData<&'video ()>,
}

assert_not_impl_all!(EventBox: Send, Sync);

impl<'video> EventBox<'video> {
    pub fn new(_: &'video Video) -> Self {
        let ret = unsafe { bind::SDL_InitSubSystem(bind::SDL_INIT_EVENTS) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl event")
        }
        Self {
            quit_event_handlers: vec![],
            window_event_handlers: vec![],
            keyboard_event_handlers: vec![],
            input_event_handlers: vec![],
            editing_event_handlers: vec![],
            mouse_event_handlers: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn handle_quit(&mut self, handler: EventHandler<'video, QuitEvent>) {
        self.quit_event_handlers.push(handler);
    }

    pub fn handle_window(&mut self, handler: EventHandler<'video, WindowEvent>) {
        self.window_event_handlers.push(handler);
    }

    pub fn handle_keyboard(&mut self, handler: EventHandler<'video, KeyboardEvent>) {
        self.keyboard_event_handlers.push(handler);
    }

    pub fn handle_input(&mut self, handler: EventHandler<'video, TextInputEvent>) {
        self.input_event_handlers.push(handler);
    }

    pub fn handle_editing(&mut self, handler: EventHandler<'video, TextEditingEvent>) {
        self.editing_event_handlers.push(handler);
    }

    pub fn handle_mouse(&mut self, handler: EventHandler<'video, MouseEvent>) {
        self.mouse_event_handlers.push(handler);
    }

    fn handle_event(&self, event: bind::SDL_Event) {
        let ty = unsafe { event.type_ };
        match ty {
            bind::SDL_EventType_SDL_QUIT => {
                let quit: QuitEvent = unsafe { event.quit }.into();
                self.quit_event_handlers
                    .iter()
                    .for_each(|handler| handler(quit.clone()))
            }
            bind::SDL_EventType_SDL_WINDOWEVENT => {
                let window: WindowEvent = unsafe { event.window }.into();
                self.window_event_handlers
                    .iter()
                    .for_each(|handler| handler(window.clone()))
            }
            bind::SDL_EventType_SDL_KEYDOWN | bind::SDL_EventType_SDL_KEYUP => {
                let keyboard: KeyboardEvent = unsafe { event.key }.into();
                self.keyboard_event_handlers
                    .iter()
                    .for_each(|handler| handler(keyboard.clone()))
            }
            bind::SDL_EventType_SDL_TEXTINPUT => {
                let input: TextInputEvent = unsafe { event.text }.into();
                self.input_event_handlers
                    .iter()
                    .for_each(|handler| handler(input.clone()))
            }
            bind::SDL_EventType_SDL_TEXTEDITING => {
                let editing: TextEditingEvent = unsafe { event.edit }.into();
                self.editing_event_handlers
                    .iter()
                    .for_each(|handler| handler(editing.clone()))
            }
            bind::SDL_EventType_SDL_MOUSEMOTION => {
                let motion: MouseMotionEvent = unsafe { event.motion }.into();
                let mouse = MouseEvent::Motion(motion);
                self.mouse_event_handlers
                    .iter()
                    .for_each(|handler| handler(mouse.clone()))
            }
            bind::SDL_EventType_SDL_MOUSEBUTTONDOWN | bind::SDL_EventType_SDL_MOUSEBUTTONUP => {
                let button: MouseButtonEvent = unsafe { event.button }.into();
                let mouse = MouseEvent::Button(button);
                self.mouse_event_handlers
                    .iter()
                    .for_each(|handler| handler(mouse.clone()))
            }
            bind::SDL_EventType_SDL_MOUSEWHEEL => {
                let wheel: MouseWheelEvent = unsafe { event.wheel }.into();
                let mouse = MouseEvent::Wheel(wheel);
                self.mouse_event_handlers
                    .iter()
                    .for_each(|handler| handler(mouse.clone()))
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
