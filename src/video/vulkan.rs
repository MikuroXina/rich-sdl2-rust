use ash::vk::{Handle as _, Instance};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, os::raw::c_uint, ptr::NonNull};

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

pub struct VkSurface<'vk> {
    ptr: NonNull<bind::VkSurfaceKHR_T>,
    _phantom: PhantomData<&'vk VkInstance<'vk>>,
}

impl<'vk> VkSurface<'vk> {
    pub fn new(vk: &'vk VkInstance<'vk>, instance: Instance) -> Result<Self> {
        let mut surface = MaybeUninit::uninit();
        let ret = unsafe {
            bind::SDL_Vulkan_CreateSurface(
                vk.window.as_ptr(),
                instance.as_raw() as *mut _,
                surface.as_mut_ptr(),
            )
        };
        if ret == bind::SDL_bool_SDL_FALSE {
            Err(SdlError::UnsupportedFeature)
        } else {
            Ok(Self {
                ptr: NonNull::new(unsafe { surface.assume_init() }).unwrap(),
                _phantom: PhantomData,
            })
        }
    }
}
