//! SDL2 event system and handlers.
//!
//! Most event structures are defined in these sub modules, but [`crate::audio::event::AudioDeviceEvent`] is defined in the external module.

use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;

use self::{
    app::QuitEvent,
    drop::DropEvent,
    game_controller::event::ControllerEvent,
    joystick::event::JoystickEvent,
    keyboard::KeyboardEvent,
    mouse::{MouseButtonEvent, MouseEvent, MouseMotionEvent, MouseWheelEvent},
    text::{TextEditingEvent, TextInputEvent},
    touch::gesture::GestureEvent,
    window::WindowEvent,
};

use crate::{audio::event::AudioDeviceEvent, bind, Sdl, Video};

pub mod app;
pub mod drop;
pub mod game_controller;
pub mod joystick;
pub mod keyboard;
pub mod mouse;
pub mod sensor;
pub mod text;
pub mod touch;
pub mod window;

/// An event handler to receive some event structure,
pub type EventHandler<'video, T> = Box<dyn Fn(&T) + 'video>;

struct EventHandlers<'video, T>(Vec<EventHandler<'video, T>>);

impl<T> Default for EventHandlers<'_, T> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<'video, T> EventHandlers<'video, T> {
    fn push(&mut self, handler: EventHandler<'video, T>) {
        self.0.push(handler);
    }

    fn call_handlers(&self, event: T) {
        self.0.iter().for_each(|handler| handler(&event));
    }
}

/// It takes the closure of handler to register, and delivers events to your event handlers by polling.
pub struct EventBox<'video> {
    quit_event_handlers: EventHandlers<'video, QuitEvent>,
    window_event_handlers: EventHandlers<'video, WindowEvent>,
    keyboard_event_handlers: EventHandlers<'video, KeyboardEvent>,
    input_event_handlers: EventHandlers<'video, TextInputEvent>,
    editing_event_handlers: EventHandlers<'video, TextEditingEvent>,
    mouse_event_handlers: EventHandlers<'video, MouseEvent>,
    controller_event_handlers: EventHandlers<'video, ControllerEvent<'video>>,
    joystick_event_handlers: EventHandlers<'video, JoystickEvent<'video>>,
    audio_device_event_handlers: EventHandlers<'video, AudioDeviceEvent>,
    drop_event_handlers: EventHandlers<'video, DropEvent>,
    gesture_event_handlers: EventHandlers<'video, GestureEvent>,
    _phantom: PhantomData<&'video ()>,
}

assert_not_impl_all!(EventBox: Send, Sync);

impl<'video> EventBox<'video> {
    /// Constructs an event box from the video system.
    pub fn new(_: &'video Video) -> Self {
        let ret = unsafe { bind::SDL_InitSubSystem(bind::SDL_INIT_EVENTS) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl event")
        }
        Self {
            quit_event_handlers: Default::default(),
            window_event_handlers: Default::default(),
            keyboard_event_handlers: Default::default(),
            input_event_handlers: Default::default(),
            editing_event_handlers: Default::default(),
            mouse_event_handlers: Default::default(),
            controller_event_handlers: Default::default(),
            joystick_event_handlers: Default::default(),
            audio_device_event_handlers: Default::default(),
            drop_event_handlers: Default::default(),
            gesture_event_handlers: Default::default(),
            _phantom: PhantomData,
        }
    }

    /// Registers the handler to handle [`QuitEvent`].
    pub fn handle_quit(&mut self, handler: EventHandler<'video, QuitEvent>) {
        self.quit_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`WindowEvent`].
    pub fn handle_window(&mut self, handler: EventHandler<'video, WindowEvent>) {
        self.window_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`KeyboardEvent`].
    pub fn handle_keyboard(&mut self, handler: EventHandler<'video, KeyboardEvent>) {
        self.keyboard_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`TextInputEvent`].
    pub fn handle_input(&mut self, handler: EventHandler<'video, TextInputEvent>) {
        self.input_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`TextEditingEvent`].
    pub fn handle_editing(&mut self, handler: EventHandler<'video, TextEditingEvent>) {
        self.editing_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`MouseEvent`].
    pub fn handle_mouse(&mut self, handler: EventHandler<'video, MouseEvent>) {
        self.mouse_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`ControllerEvent`].
    pub fn handle_controller(&mut self, handler: EventHandler<'video, ControllerEvent<'video>>) {
        self.controller_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`JoystickEvent`].
    pub fn handle_joystick(&mut self, handler: EventHandler<'video, JoystickEvent<'video>>) {
        self.joystick_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`AudioDeviceEvent`].
    pub fn handle_audio_device(&mut self, handler: EventHandler<'video, AudioDeviceEvent>) {
        self.audio_device_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`DropEvent`].
    pub fn handle_drop(&mut self, handler: EventHandler<'video, DropEvent>) {
        self.drop_event_handlers.push(handler);
    }

    /// Registers the handler to handle [`GestureEvent`].
    pub fn handle_gesture(&mut self, handler: EventHandler<'video, GestureEvent>) {
        self.gesture_event_handlers.push(handler);
    }

    fn handle_event(&self, event: bind::SDL_Event) {
        let ty = unsafe { event.type_ };
        match ty {
            bind::SDL_EventType_SDL_QUIT => {
                let quit: QuitEvent = unsafe { event.quit }.into();
                self.quit_event_handlers.call_handlers(quit);
            }
            bind::SDL_EventType_SDL_WINDOWEVENT => {
                let window: WindowEvent = unsafe { event.window }.into();
                self.window_event_handlers.call_handlers(window);
            }
            bind::SDL_EventType_SDL_KEYDOWN | bind::SDL_EventType_SDL_KEYUP => {
                let keyboard: KeyboardEvent = unsafe { event.key }.into();
                self.keyboard_event_handlers.call_handlers(keyboard);
            }
            bind::SDL_EventType_SDL_TEXTINPUT => {
                let input: TextInputEvent = unsafe { event.text }.into();
                self.input_event_handlers.call_handlers(input);
            }
            bind::SDL_EventType_SDL_TEXTEDITING => {
                let editing: TextEditingEvent = unsafe { event.edit }.into();
                self.editing_event_handlers.call_handlers(editing);
            }
            bind::SDL_EventType_SDL_MOUSEMOTION => {
                let motion: MouseMotionEvent = unsafe { event.motion }.into();
                let mouse = MouseEvent::Motion(motion);
                self.mouse_event_handlers.call_handlers(mouse);
            }
            bind::SDL_EventType_SDL_MOUSEBUTTONDOWN | bind::SDL_EventType_SDL_MOUSEBUTTONUP => {
                let button: MouseButtonEvent = unsafe { event.button }.into();
                let mouse = MouseEvent::Button(button);
                self.mouse_event_handlers.call_handlers(mouse);
            }
            bind::SDL_EventType_SDL_MOUSEWHEEL => {
                let wheel: MouseWheelEvent = unsafe { event.wheel }.into();
                let mouse = MouseEvent::Wheel(wheel);
                self.mouse_event_handlers.call_handlers(mouse);
            }
            bind::SDL_EventType_SDL_CONTROLLERAXISMOTION => {
                let con: ControllerEvent = unsafe { event.caxis }.into();
                self.controller_event_handlers.call_handlers(con);
            }
            bind::SDL_EventType_SDL_CONTROLLERBUTTONDOWN
            | bind::SDL_EventType_SDL_CONTROLLERBUTTONUP => {
                let con: ControllerEvent = unsafe { event.cbutton }.into();
                self.controller_event_handlers.call_handlers(con);
            }
            bind::SDL_EventType_SDL_CONTROLLERDEVICEADDED
            | bind::SDL_EventType_SDL_CONTROLLERDEVICEREMOVED
            | bind::SDL_EventType_SDL_CONTROLLERDEVICEREMAPPED => {
                let con: ControllerEvent = unsafe { event.cdevice }.into();
                self.controller_event_handlers.call_handlers(con);
            }
            bind::SDL_EventType_SDL_JOYAXISMOTION => {
                let joy: JoystickEvent = unsafe { event.jaxis }.into();
                self.joystick_event_handlers.call_handlers(joy);
            }
            bind::SDL_EventType_SDL_JOYBALLMOTION => {
                let joy: JoystickEvent = unsafe { event.jball }.into();
                self.joystick_event_handlers.call_handlers(joy);
            }
            bind::SDL_EventType_SDL_JOYBUTTONDOWN | bind::SDL_EventType_SDL_JOYBUTTONUP => {
                let joy: JoystickEvent = unsafe { event.jbutton }.into();
                self.joystick_event_handlers.call_handlers(joy);
            }
            bind::SDL_EventType_SDL_JOYDEVICEADDED | bind::SDL_EventType_SDL_JOYDEVICEREMOVED => {
                let joy: JoystickEvent = unsafe { event.jdevice }.into();
                self.joystick_event_handlers.call_handlers(joy);
            }
            bind::SDL_EventType_SDL_JOYHATMOTION => {
                let joy: JoystickEvent = unsafe { event.jhat }.into();
                self.joystick_event_handlers.call_handlers(joy);
            }
            bind::SDL_EventType_SDL_AUDIODEVICEADDED
            | bind::SDL_EventType_SDL_AUDIODEVICEREMOVED => {
                let audio = unsafe { event.adevice }.into();
                self.audio_device_event_handlers.call_handlers(audio);
            }
            bind::SDL_EventType_SDL_DROPFILE
            | bind::SDL_EventType_SDL_DROPTEXT
            | bind::SDL_EventType_SDL_DROPBEGIN
            | bind::SDL_EventType_SDL_DROPCOMPLETE => {
                let drop = unsafe { event.drop }.into();
                self.drop_event_handlers.call_handlers(drop);
            }
            bind::SDL_EventType_SDL_MULTIGESTURE => {
                let gesture = unsafe { event.mgesture }.into();
                self.gesture_event_handlers.call_handlers(gesture);
            }
            bind::SDL_EventType_SDL_DOLLARGESTURE => {
                let gesture = unsafe { event.dgesture }.into();
                self.gesture_event_handlers.call_handlers(gesture);
            }
            _ => {}
        }
    }

    /// Polling the events and triggers the event handlers.
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

    /// Waits until the next event occurs, but unlock with timeout seconds.
    pub fn wait_next_event_with(&self, timeout_ms: u32) {
        use std::mem::MaybeUninit;
        let mut event = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_WaitEventTimeout(event.as_mut_ptr(), timeout_ms as i32) };
        if ret == 0 {
            return;
        }
        let event = unsafe { event.assume_init() };
        self.handle_event(event);
    }
}

impl<'video> Drop for EventBox<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_EVENTS) }
    }
}
