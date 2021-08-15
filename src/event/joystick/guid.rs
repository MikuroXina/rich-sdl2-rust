//! The GUID of joystick devices.

use std::{
    ffi::{CStr, CString},
    os::raw::c_int,
};

use crate::bind;

/// A GUID associated with joystick devices.
#[derive(Debug, Clone)]
pub struct Guid([u8; 16]);

impl Guid {
    /// Returns mapping string for the game controller having the GUID.
    pub fn mapping(&self) -> String {
        let ptr = unsafe {
            bind::SDL_GameControllerMappingForGUID(bind::SDL_JoystickGUID { data: self.0 })
        };
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let ret = cstr.to_string_lossy().to_string();
        unsafe { bind::SDL_free(ptr.cast()) };
        ret
    }
}

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = [0u8; 33];
        unsafe {
            bind::SDL_JoystickGetGUIDString(
                bind::SDL_JoystickGUID { data: self.0 },
                buf.as_mut_ptr().cast(),
                buf.len() as c_int,
            )
        }
        write!(f, "{}", String::from_utf8_lossy(&buf))
    }
}

/// An error to tell the failure on parsing `Guid` from string, showing that it is invalid length to convert into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidLengthError {
    actual_length: usize,
}

impl std::fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "length must equals to 16 but actual length was {}",
            self.actual_length
        )
    }
}

impl std::error::Error for InvalidLengthError {}

impl std::str::FromStr for Guid {
    type Err = InvalidLengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 16 {
            return Err(InvalidLengthError {
                actual_length: s.len(),
            });
        }
        let c_str = CString::new(s).expect("invalid string");
        let raw_guid = unsafe { bind::SDL_JoystickGetGUIDFromString(c_str.as_ptr()) };
        Ok(raw_guid.into())
    }
}

impl From<bind::SDL_JoystickGUID> for Guid {
    fn from(raw: bind::SDL_JoystickGUID) -> Self {
        Self(raw.data)
    }
}
