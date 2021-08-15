//! Definitions of key modification codes.

use bitflags::bitflags;

use crate::bind;

bitflags! {
    /// A key modification flag.
    pub struct KeyMod : u16 {
        /// No mod key pressed.
        const NONE = bind::SDL_Keymod_KMOD_NONE as u16;
        /// Left shift pressed.
        const LSHIFT = bind::SDL_Keymod_KMOD_LSHIFT as u16;
        /// Right shift pressed.
        const RSHIFT = bind::SDL_Keymod_KMOD_RSHIFT as u16;
        /// Left control pressed.
        const LCTRL = bind::SDL_Keymod_KMOD_LCTRL as u16;
        /// Right control pressed.
        const RCTRL = bind::SDL_Keymod_KMOD_RCTRL as u16;
        /// Left alt pressed.
        const LALT = bind::SDL_Keymod_KMOD_LALT as u16;
        /// Right alt pressed.
        const RALT = bind::SDL_Keymod_KMOD_RALT as u16;
        /// Left meta pressed.
        const LGUI = bind::SDL_Keymod_KMOD_LGUI as u16;
        /// Right meta pressed.
        const RGUI = bind::SDL_Keymod_KMOD_RGUI as u16;
        /// Num lock pressed.
        const NUM = bind::SDL_Keymod_KMOD_NUM as u16;
        /// Caps lock pressed.
        const CAPS = bind::SDL_Keymod_KMOD_CAPS as u16;
        /// Alt graphic pressed.
        const MODE = bind::SDL_Keymod_KMOD_MODE as u16;
        /// Left or right control pressed.
        const CTRL = bind::SDL_Keymod_KMOD_CTRL as u16;
        /// Left or right shift pressed.
        const SHIFT = bind::SDL_Keymod_KMOD_SHIFT as u16;
        /// Left or right alt pressed.
        const ALT = bind::SDL_Keymod_KMOD_ALT as u16;
        /// Left or right meta pressed.
        const GUI = bind::SDL_Keymod_KMOD_GUI as u16;
    }
}

impl KeyMod {
    /// Returns the current modification status on the keyboard.
    pub fn current() -> Self {
        Self::from_bits(unsafe { bind::SDL_GetModState() } as u16).unwrap()
    }
}
