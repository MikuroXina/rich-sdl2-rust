use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use crate::{bind, event::joystick::Joystick};

use super::Haptic;

pub struct JoystickHaptic<'joystick> {
    haptic: Haptic,
    _phantom: PhantomData<&'joystick Joystick>,
}

impl<'joystick> JoystickHaptic<'joystick> {
    fn from(joystick: impl AsRef<Joystick> + 'joystick) -> Option<Self> {
        let is_supported = unsafe {
            bind::SDL_JoystickIsHaptic(joystick.as_ref().ptr().as_ptr()) as bind::SDL_bool
                == bind::SDL_bool_SDL_TRUE
        };
        if !is_supported {
            return None;
        }
        let ptr = unsafe { bind::SDL_HapticOpenFromJoystick(joystick.as_ref().ptr().as_ptr()) };
        Some(Self {
            haptic: Haptic {
                ptr: NonNull::new(ptr).unwrap(),
            },
            _phantom: PhantomData,
        })
    }
}

impl Deref for JoystickHaptic<'_> {
    type Target = Haptic;

    fn deref(&self) -> &Self::Target {
        &self.haptic
    }
}

impl Drop for JoystickHaptic<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_HapticClose(self.ptr.as_ptr()) }
    }
}
