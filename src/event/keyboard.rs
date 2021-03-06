use key_code::KeyCode;
use key_mod::KeyMod;
use scan_code::ScanCode;

use crate::bind;

pub mod key_code;
pub mod key_mod;
pub mod scan_code;

#[derive(Debug, Clone)]
pub struct KeySymbol {
    scan_code: ScanCode,
    key_code: KeyCode,
    key_mod: KeyMod,
}

impl From<bind::SDL_Keysym> for KeySymbol {
    fn from(sym: bind::SDL_Keysym) -> Self {
        Self {
            scan_code: sym.scancode.into(),
            key_code: sym.sym.into(),
            key_mod: KeyMod::from_bits(sym.mod_).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub timestamp: u32,
    pub window_id: u32,
    pub is_pressed: bool,
    pub is_repeated: bool,
    pub symbol: KeySymbol,
}

impl From<bind::SDL_KeyboardEvent> for KeyboardEvent {
    fn from(raw: bind::SDL_KeyboardEvent) -> Self {
        Self {
            timestamp: raw.timestamp,
            window_id: raw.windowID,
            is_pressed: raw.state as u32 == bind::SDL_PRESSED,
            is_repeated: raw.repeat != 0,
            symbol: raw.keysym.into(),
        }
    }
}
