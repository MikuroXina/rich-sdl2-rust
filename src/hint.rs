use std::{
    ffi::{CStr, CString},
    os::raw::c_uint,
};

use crate::{bind, Result, SdlError};

pub fn get_hint(key: &str) -> Option<String> {
    let cstr = CString::new(key).expect("key must not be empty");
    let hint = unsafe { bind::SDL_GetHint(cstr.as_ptr()) };
    (!hint.is_null()).then(|| {
        unsafe { CStr::from_ptr(hint) }
            .to_string_lossy()
            .to_string()
    })
}

pub fn set_hint(key: &str, value: &str) -> Result<()> {
    let key_cstr = CString::new(key).expect("key must not be empty");
    let value_cstr = CString::new(value).expect("value must not be empty");
    let ret = unsafe { bind::SDL_SetHint(key_cstr.as_ptr(), value_cstr.as_ptr()) };
    if ret == bind::SDL_bool_SDL_TRUE {
        Ok(())
    } else {
        Err(SdlError::UnsupportedFeature)
    }
}

pub enum HintPriority {
    Default,
    Normal,
    Override,
}

impl HintPriority {
    fn into_raw(self) -> c_uint {
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

pub fn set_hint_with_priority(key: &str, value: &str, priority: HintPriority) -> Result<()> {
    let key_cstr = CString::new(key).expect("key must not be empty");
    let value_cstr = CString::new(value).expect("value must not be empty");
    let ret = unsafe {
        bind::SDL_SetHintWithPriority(key_cstr.as_ptr(), value_cstr.as_ptr(), priority.into_raw())
    };
    if ret == bind::SDL_bool_SDL_TRUE {
        Ok(())
    } else {
        Err(SdlError::UnsupportedFeature)
    }
}

pub fn clear_hints() {
    unsafe { bind::SDL_ClearHints() }
}
