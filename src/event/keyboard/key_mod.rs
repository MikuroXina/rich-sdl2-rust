//! Definitions of key modification codes.

use bitflags::bitflags;

use crate::bind;

bitflags! {
    /// A key modification flag.
    pub struct KeyMod : u16 {
        /// No mod key pressed.
        const NONE = bind::KMOD_NONE as u16;
        /// Left shift pressed.
        const LSHIFT = bind::KMOD_LSHIFT as u16;
        /// Right shift pressed.
        const RSHIFT = bind::KMOD_RSHIFT as u16;
        /// Left control pressed.
        const LCTRL = bind::KMOD_LCTRL as u16;
        /// Right control pressed.
        const RCTRL = bind::KMOD_RCTRL as u16;
        /// Left alt pressed.
        const LALT = bind::KMOD_LALT as u16;
        /// Right alt pressed.
        const RALT = bind::KMOD_RALT as u16;
        /// Left meta pressed.
        const LGUI = bind::KMOD_LGUI as u16;
        /// Right meta pressed.
        const RGUI = bind::KMOD_RGUI as u16;
        /// Num lock pressed.
        const NUM = bind::KMOD_NUM as u16;
        /// Caps lock pressed.
        const CAPS = bind::KMOD_CAPS as u16;
        /// Alt graphic pressed.
        const MODE = bind::KMOD_MODE as u16;
        /// Left or right control pressed.
        const CTRL = bind::KMOD_CTRL as u16;
        /// Left or right shift pressed.
        const SHIFT = bind::KMOD_SHIFT as u16;
        /// Left or right alt pressed.
        const ALT = bind::KMOD_ALT as u16;
        /// Left or right meta pressed.
        const GUI = bind::KMOD_GUI as u16;
    }
}

impl KeyMod {
    /// Returns the current modification status on the keyboard.
    pub fn current() -> Self {
        Self::from_bits(unsafe { bind::SDL_GetModState() } as u16).unwrap()
    }
}
