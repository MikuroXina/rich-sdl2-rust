#![allow(unused)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![cfg_attr(feature = "simd_allocator", feature(allocator_api))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

//! # rich-sdl2-rust
//!
//! The rich-sdl2-rust provides wrapper for SDL2 and abstractions of that APIs, [`audio`], [`window`], [`EventBox`] and so on.
//!
//! ## Module Navigation
//!
//! - [Window and Graphics](window)
//! - [Simple Audio Control](audio)
//! - [Handling Events](EventBox)
//! - ...
//!
//! ## Crate features
//!
//! - `vulkan`: The Vulkan support API wrapper.
//! - `nightly`: The features can be used on nightly.
//!   - `simd_allocator`: The wrapper of SIMD-friendly allocator.

pub mod audio;
mod error;
pub mod event;
pub mod file;
pub mod haptic;
pub mod hint;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "mixer")]
pub mod mixer;
pub mod power;
mod sdl;
pub mod system;
mod timer;
#[cfg(feature = "ttf")]
pub mod ttf;
mod video;

use rich_sdl2_rust_sys as bind;

pub use error::*;
pub use event::{app, EventBox};
pub use sdl::*;
pub use timer::*;
pub use video::*;

#[cfg(not(target_env = "msvc"))]
type EnumInt = std::os::raw::c_uint;
#[cfg(target_env = "msvc")]
type EnumInt = std::os::raw::c_int;

/// Converts an option reference into a constant raw pointer.
///
/// # Safety
///
/// The object of `opt` must live over usage of a returned pointer. Otherwise it will occur UB.
pub(crate) unsafe fn as_raw<T>(opt: &Option<T>) -> *const T {
    opt.as_ref().map_or(std::ptr::null(), |x| &*x)
}

/// Converts an option reference into a mutable raw pointer.
///
/// # Safety
///
/// The object of `opt` must live over usage of a returned pointer. Otherwise it will occur UB.
pub(crate) unsafe fn as_raw_mut<T>(opt: &mut Option<T>) -> *mut T {
    opt.as_mut().map_or(std::ptr::null_mut(), |x| &mut *x)
}
