use std::ffi::{CStr, CString};

use crate::bind;

pub mod mode;
mod rw;

pub use rw::*;

pub fn base_path() -> String {
    let cstr = unsafe { CStr::from_ptr(bind::SDL_GetBasePath()) };
    cstr.to_string_lossy().to_string()
}

pub fn pref_path(org: &str, app: &str) -> String {
    let org_cstr = CString::new(org).unwrap();
    let app_cstr = CString::new(app).unwrap();
    let cstr =
        unsafe { CStr::from_ptr(bind::SDL_GetPrefPath(org_cstr.as_ptr(), app_cstr.as_ptr())) };
    cstr.to_string_lossy().to_string()
}
