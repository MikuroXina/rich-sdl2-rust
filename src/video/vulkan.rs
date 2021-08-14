use ash::vk::{Handle as _, Instance};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, os::raw::c_uint, ptr::NonNull};

use crate::{
    bind,
    geo::Size,
    window::{Window, WindowContextKind},
    Result, SdlError,
};

#[derive(Debug)]
pub struct VkInstance<'window> {
    window: &'window Window<'window>,
    extensions: Vec<String>,
}

impl<'window> VkInstance<'window> {
    pub fn new(window: &'window Window<'window>) -> Result<Self> {
        if window.state().context_kind != WindowContextKind::Vulkan {
            return Err(SdlError::UnsupportedFeature);
        }
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

    pub fn drawable_size(&self) -> Size {
        let mut width = 0;
        let mut height = 0;
        unsafe {
            bind::SDL_Vulkan_GetDrawableSize(
                self.window.as_ptr(),
                &mut width as _,
                &mut height as _,
            )
        }
        Size {
            width: width as _,
            height: height as _,
        }
    }
}

pub struct VkSurface<'vk> {
    ptr: NonNull<bind::VkSurfaceKHR_T>,
    _phantom: PhantomData<&'vk VkInstance<'vk>>,
}

impl std::fmt::Debug for VkSurface<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VkSurface").finish()
    }
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
