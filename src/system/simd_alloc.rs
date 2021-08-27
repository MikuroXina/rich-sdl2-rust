//! Provides SIMD-friendly allocator from SDL2.

#![cfg(feature = "simd_allocator")]
#![doc(cfg(feature = "simd_allocator"))]

use std::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

use crate::bind;

/// Allocates memory in a SIMD-friendly way.
pub struct SimdAllocator;

unsafe impl Allocator for SimdAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { bind::SDL_SIMDAlloc(layout.size() as _) };
        if ptr.is_null() {
            return Err(AllocError);
        }
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr.cast(), layout.size()) };
        NonNull::new(slice).ok_or(AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        bind::SDL_SIMDFree(ptr.as_ptr().cast());
    }
}
