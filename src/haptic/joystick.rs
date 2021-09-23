//! A haptic device integrated with the joystick.

use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use crate::{bind, event::joystick::Joystick};

use super::Haptic;

/// A haptic device got from the joystick.
pub struct JoystickHaptic<'joystick> {
    haptic: Haptic,
    _phantom: PhantomData<&'joystick Joystick>,
}

impl std::fmt::Debug for JoystickHaptic<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JoystickHaptic")
            .field("haptic", &self.haptic)
            .finish()
    }
}

impl<'joystick> JoystickHaptic<'joystick> {
    /// Constructs from a reference to [`Joystick`].
    pub fn new(joystick: impl AsRef<Joystick> + 'joystick) -> Option<Self> {
        let is_supported = unsafe {
            bind::SDL_JoystickIsHaptic(joystick.as_ref().ptr().as_ptr()) as bind::SDL_bool
                == bind::SDL_TRUE
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
