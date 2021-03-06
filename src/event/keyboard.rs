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
