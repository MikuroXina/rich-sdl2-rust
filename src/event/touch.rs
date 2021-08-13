use static_assertions::assert_not_impl_all;
use std::{cell::Cell, marker::PhantomData, ptr::NonNull};

use crate::bind;

pub struct TouchFinger<'device> {
    ptr: NonNull<bind::SDL_Finger>,
    _phantom: PhantomData<&'device ()>,
}

pub struct TouchDevice(i64, PhantomData<Cell<u8>>);

assert_not_impl_all!(TouchDevice: Send, Sync);

impl TouchDevice {
    pub fn all_devices() -> Vec<Self> {
        let num = unsafe { bind::SDL_GetNumTouchDevices() };
        (0..num)
            .map(|index| {
                let id = unsafe { bind::SDL_GetTouchDevice(index) };
                Self(id, PhantomData)
            })
            .collect()
    }

    pub fn record(&self) -> bool {
        unsafe { bind::SDL_RecordGesture(self.0) == 1 }
    }

    pub fn touch_fingers(&self) -> Vec<TouchFinger> {
        let num = unsafe { bind::SDL_GetNumTouchFingers(self.0) };
        (0..num)
            .flat_map(|index| {
                let ptr = unsafe { bind::SDL_GetTouchFinger(self.0, index) };
                (!ptr.is_null()).then(|| TouchFinger {
                    ptr: NonNull::new(ptr).unwrap(),
                    _phantom: PhantomData,
                })
            })
            .collect()
    }
}
