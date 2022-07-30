use std::{
    ffi::{c_void, CStr, CString},
    marker::PhantomData,
    os::raw::c_char,
};

use crate::bind;

/// An event on updated the hint.
#[derive(Debug, Clone)]
pub struct HintEvent<'a> {
    name: &'a str,
    old_value: &'a str,
    new_value: &'a str,
}

/// A callback invoked on updated the hint.
pub trait HintCallback<'callback>: FnMut(HintEvent<'callback>) + 'callback {}

/// An hint update observer.
pub struct HintObserver<'callback, T: HintCallback<'callback>> {
    key: CString,
    callback: &'callback mut T,
}

impl<'c, T: HintCallback<'c>> std::fmt::Debug for HintObserver<'c, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HintObserver")
            .field("key", &self.key)
            .finish_non_exhaustive()
    }
}

impl<'c, T: HintCallback<'c>> HintObserver<'c, T> {
    /// Constructs an observer to observe by the key and callback.
    pub fn new(key: &str, callback: &'c mut T) -> Self {
        let key = CString::new(key).expect("key must not be empty");
        unsafe {
            bind::SDL_AddHintCallback(
                key.as_ptr(),
                Some(hint_observer_wrap_handler::<T>),
                (callback as *mut T).cast(),
            );
        }
        Self { key, callback }
    }
}

impl<'c, T: HintCallback<'c>> Drop for HintObserver<'c, T> {
    fn drop(&mut self) {
        unsafe {
            bind::SDL_DelHintCallback(
                self.key.as_ptr(),
                Some(hint_observer_wrap_handler::<T>),
                (self.callback as *mut T).cast(),
            );
        }
    }
}

extern "C" fn hint_observer_wrap_handler<'callback, T: HintCallback<'callback>>(
    userdata: *mut c_void,
    name: *const c_char,
    old_value: *const c_char,
    new_value: *const c_char,
) {
    let callback = unsafe { &mut *userdata.cast::<T>() };
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    let old_value = unsafe { CStr::from_ptr(old_value) }.to_str().unwrap();
    let new_value = unsafe { CStr::from_ptr(new_value) }.to_str().unwrap();
    callback(HintEvent {
        name,
        old_value,
        new_value,
    });
}
