use bitflags::bitflags;

use crate::bind;

bitflags! {
  pub struct KeyMod : u32 {
    const NONE = bind::SDL_Keymod_KMOD_NONE;
    const LSHIFT = bind::SDL_Keymod_KMOD_LSHIFT;
    const RSHIFT = bind::SDL_Keymod_KMOD_RSHIFT;
    const LCTRL = bind::SDL_Keymod_KMOD_LCTRL;
    const RCTRL = bind::SDL_Keymod_KMOD_RCTRL;
    const LALT = bind::SDL_Keymod_KMOD_LALT;
    const RALT = bind::SDL_Keymod_KMOD_RALT;
    const LGUI = bind::SDL_Keymod_KMOD_LGUI;
    const RGUI = bind::SDL_Keymod_KMOD_RGUI;
    const NUM = bind::SDL_Keymod_KMOD_NUM;
    const CAPS = bind::SDL_Keymod_KMOD_CAPS;
    const MODE = bind::SDL_Keymod_KMOD_MODE;
    const RESERVED = bind::SDL_Keymod_KMOD_RESERVED;
    const CTRL = bind::SDL_Keymod_KMOD_CTRL;
    const SHIFT = bind::SDL_Keymod_KMOD_SHIFT;
    const ALT = bind::SDL_Keymod_KMOD_ALT;
    const GUI = bind::SDL_Keymod_KMOD_GUI;
  }
}
