use std::{ffi::CStr, os::raw::c_uint};

use crate::{bind, window::Window, Result, SdlError};

pub struct VkInstance<'window> {
    window: &'window Window<'window>,
    extensions: Vec<String>,
}

impl<'window> VkInstance<'window> {
    pub fn new(window: &'window Window<'window>) -> Result<Self> {
        let mut num: c_uint = 0;
        let ret = unsafe {
            bind::SDL_Vulkan_GetInstanceExtensions(
                window.as_ptr(),
                &mut num as *mut _,
                std::ptr::null_mut(),
            )
        };
        if ret == bind::SDL_bool_SDL_FALSE {
            return Err(SdlError::UnsupportedFeature);
        }
        let mut extensions = vec![std::ptr::null(); num as usize];
        let _ = unsafe {
            bind::SDL_Vulkan_GetInstanceExtensions(
                window.as_ptr(),
                &mut num as *mut _,
                extensions.as_mut_ptr(),
            )
        };
        let extensions = extensions
            .into_iter()
            .map(|ptr| unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into())
            .collect();
        Ok(Self { window, extensions })
    }

    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }
}
