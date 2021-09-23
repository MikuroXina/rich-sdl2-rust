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
mod bind;
mod error;
pub mod event;
pub mod file;
pub mod haptic;
pub mod hint;
pub mod power;
mod sdl;
pub mod system;
mod timer;
mod video;

pub use error::*;
pub use event::{app, EventBox};
pub use sdl::*;
pub use timer::*;
pub use video::*;

#[cfg(not(target_env = "msvc"))]
type EnumInt = std::os::raw::c_uint;
#[cfg(target_env = "msvc")]
type EnumInt = std::os::raw::c_int;
