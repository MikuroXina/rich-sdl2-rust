//! Provides SIMD-friendly allocator from SDL2.

#![feature(allocator_api)]
#![cfg(feature = "simd_allocator")]

use std::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

use crate::bind;

/// Allocates memory in a SIMD-friendly way.
pub struct SimdAllocator;

unsafe impl Allocator for SimdAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { bind::SDL_SIMDAlloc(layout.size()) };
        NonNull::new(ptr.cast()).unwrap_or(AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        bind::SDL_SIMDFree(ptr.as_ptr());
    }
}
