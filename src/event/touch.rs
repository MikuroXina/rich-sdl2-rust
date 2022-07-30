//! Controls touch devices, fingers, and gestures.

use static_assertions::assert_not_impl_all;
use std::{cell::Cell, marker::PhantomData, ptr::NonNull};

use crate::{bind, file::RwOps, Result, Sdl, SdlError};

pub mod gesture;

/// A finger input, which having coordinates and pressures.
pub struct TouchFinger<'device> {
    ptr: NonNull<bind::SDL_Finger>,
    _phantom: PhantomData<&'device ()>,
}

impl std::fmt::Debug for TouchFinger<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TouchFinger")
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}

impl TouchFinger<'_> {
    /// Returns the id of the finger.
    #[must_use]
    pub fn id(&self) -> i64 {
        unsafe { self.ptr.as_ref() }.id
    }
    /// Returns the x pos of the finger.
    #[must_use]
    pub fn x(&self) -> f32 {
        unsafe { self.ptr.as_ref() }.x
    }
    /// Returns the y pos of the finger.
    #[must_use]
    pub fn y(&self) -> f32 {
        unsafe { self.ptr.as_ref() }.y
    }
    /// Returns the pressures of the finger.
    #[must_use]
    pub fn pressure(&self) -> f32 {
        unsafe { self.ptr.as_ref() }.pressure
    }
}

/// A device that receives the finger input.
#[derive(Debug, Clone)]
pub struct TouchDevice(bind::SDL_TouchID, PhantomData<Cell<u8>>);

assert_not_impl_all!(TouchDevice: Send, Sync);

impl TouchDevice {
    /// Setup the system and recognize all touch devices.
    #[must_use]
    pub fn all_devices() -> Vec<Self> {
        let num = unsafe { bind::SDL_GetNumTouchDevices() };
        (0..num)
            .map(|index| {
                let id = unsafe { bind::SDL_GetTouchDevice(index) };
                Self(id, PhantomData)
            })
            .collect()
    }

    /// Starts to record the gesture. After invoking this and record, then an event [`gesture::GestureEvent::DollarRecord`] will occur. Please handle the event.
    #[must_use]
    pub fn record(&self) -> bool {
        unsafe { bind::SDL_RecordGesture(self.0) == 1 }
    }

    /// Returns all the touching fingers.
    #[must_use]
    pub fn touch_fingers(&self) -> Vec<TouchFinger> {
        let num = unsafe { bind::SDL_GetNumTouchFingers(self.0) };
        (0..num)
            .filter_map(|index| {
                let ptr = unsafe { bind::SDL_GetTouchFinger(self.0, index) };
                (!ptr.is_null()).then(|| TouchFinger {
                    ptr: NonNull::new(ptr).unwrap(),
                    _phantom: PhantomData,
                })
            })
            .collect()
    }

    /// Loads $1 template from `src` and returns the numbers of loaded templates.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to load template data from `src`.
    pub fn load_dollar_templates(&self, src: &RwOps) -> Result<usize> {
        let ret = unsafe { bind::SDL_LoadDollarTemplates(self.0, src.ptr().as_ptr()) };
        if ret <= 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as usize)
        }
    }
}
