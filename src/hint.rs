//! Getting/setting hints for SDL2 features working.

use std::{
    ffi::{CStr, CString},
    os::raw::c_uint,
};

use crate::{bind, EnumInt, Result, SdlError};

mod observer;

pub use observer::*;

/// Returns the hint value of the key if exists.
#[must_use]
pub fn get_hint(key: &str) -> Option<String> {
    let cstr = CString::new(key).expect("key must not be empty");
    let hint = unsafe { bind::SDL_GetHint(cstr.as_ptr()) };
    (!hint.is_null()).then(|| {
        unsafe { CStr::from_ptr(hint) }
            .to_string_lossy()
            .to_string()
    })
}

/// Returns the boolean hint value of the key if exists.
#[must_use]
pub fn get_hint_bool(key: &str) -> Option<bool> {
    let cstr = CString::new(key).expect("key must not be empty");
    let ret = unsafe { bind::SDL_GetHintBoolean(cstr.as_ptr(), 2) };
    (ret != 2).then(|| ret == bind::SDL_TRUE)
}

/// Sets the hint value of the key, or `Err` if does not exist.
///
/// # Panics
///
/// Panics if `key` or `value` was empty.
///
/// # Errors
///
/// Returns `Err` if the hint of `key` is unsupported.
pub fn set_hint(key: &str, value: &str) -> Result<()> {
    let key_cstr = CString::new(key).expect("key must not be empty");
    let value_cstr = CString::new(value).expect("value must not be empty");
    let ret = unsafe { bind::SDL_SetHint(key_cstr.as_ptr(), value_cstr.as_ptr()) };
    if ret == bind::SDL_TRUE {
        Ok(())
    } else {
        Err(SdlError::UnsupportedFeature)
    }
}

/// A priority of the hint specifying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HintPriority {
    /// A default priority, low.
    Default,
    /// A medium priority.
    Normal,
    /// A higher priority.
    Override,
}

impl HintPriority {
    fn into_raw(self) -> EnumInt {
        match self {
            HintPriority::Default => 0,
            HintPriority::Normal => 1,
            HintPriority::Override => 2,
        }
    }
}

impl Default for HintPriority {
    fn default() -> Self {
        Self::Default
    }
}

/// Sets a hint value of the key with a priority.
///
/// # Errors
///
/// Returns `Err` if the hint of `key` does not exist.
pub fn set_hint_with_priority(key: &str, value: &str, priority: HintPriority) -> Result<()> {
    let key_cstr = CString::new(key).expect("key must not be empty");
    let value_cstr = CString::new(value).expect("value must not be empty");
    let ret = unsafe {
        bind::SDL_SetHintWithPriority(key_cstr.as_ptr(), value_cstr.as_ptr(), priority.into_raw())
    };
    if ret == bind::SDL_TRUE {
        Ok(())
    } else {
        Err(SdlError::UnsupportedFeature)
    }
}

/// Clears all the set hints.
pub fn clear_hints() {
    unsafe { bind::SDL_ClearHints() }
}
