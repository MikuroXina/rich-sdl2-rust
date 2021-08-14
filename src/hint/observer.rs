use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
};

use crate::bind;

pub struct HintEvent<'a> {
    name: &'a str,
    old_value: &'a str,
    new_value: &'a str,
}

pub type HintCallback<'callback> = Box<dyn FnMut(HintEvent<'callback>) + 'callback>;

pub struct HintObserver<'callback> {
    key: CString,
    callback_raw: *mut HintCallback<'callback>,
}

impl<'callback> HintObserver<'callback> {
    pub fn new(key: &str, callback: HintCallback<'callback>) -> Self {
        let key = CString::new(key).expect("key must not be empty");
        let wrapped = Box::new(callback);
        let callback_raw = Box::into_raw(wrapped);
        unsafe {
            bind::SDL_AddHintCallback(
                key.as_ptr(),
                Some(hint_observer_wrap_handler),
                callback_raw.cast(),
            )
        }
        Self { key, callback_raw }
    }
}

impl Drop for HintObserver<'_> {
    fn drop(&mut self) {
        unsafe {
            bind::SDL_DelHintCallback(
                self.key.as_ptr(),
                Some(hint_observer_wrap_handler),
                self.callback_raw.cast(),
            )
        }
        let _ = unsafe { Box::from_raw(self.callback_raw) };
    }
}

extern "C" fn hint_observer_wrap_handler(
    userdata: *mut c_void,
    name: *const c_char,
    old_value: *const c_char,
    new_value: *const c_char,
) {
    let callback = unsafe { &mut *(userdata as *mut HintCallback) };
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    let old_value = unsafe { CStr::from_ptr(old_value) }.to_str().unwrap();
    let new_value = unsafe { CStr::from_ptr(new_value) }.to_str().unwrap();
    callback(HintEvent {
        name,
        old_value,
        new_value,
    });
}
