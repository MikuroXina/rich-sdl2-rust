use bitflags::bitflags;

use crate::bind;

bitflags! {
  pub struct KeyMod : u16 {
    const NONE = bind::SDL_Keymod_KMOD_NONE as u16;
    const LSHIFT = bind::SDL_Keymod_KMOD_LSHIFT as u16;
    const RSHIFT = bind::SDL_Keymod_KMOD_RSHIFT as u16;
    const LCTRL = bind::SDL_Keymod_KMOD_LCTRL as u16;
    const RCTRL = bind::SDL_Keymod_KMOD_RCTRL as u16;
    const LALT = bind::SDL_Keymod_KMOD_LALT as u16;
    const RALT = bind::SDL_Keymod_KMOD_RALT as u16;
    const LGUI = bind::SDL_Keymod_KMOD_LGUI as u16;
    const RGUI = bind::SDL_Keymod_KMOD_RGUI as u16;
    const NUM = bind::SDL_Keymod_KMOD_NUM as u16;
    const CAPS = bind::SDL_Keymod_KMOD_CAPS as u16;
    const MODE = bind::SDL_Keymod_KMOD_MODE as u16;
    const RESERVED = bind::SDL_Keymod_KMOD_RESERVED as u16;
    const CTRL = bind::SDL_Keymod_KMOD_CTRL as u16;
    const SHIFT = bind::SDL_Keymod_KMOD_SHIFT as u16;
    const ALT = bind::SDL_Keymod_KMOD_ALT as u16;
    const GUI = bind::SDL_Keymod_KMOD_GUI as u16;
  }
}
