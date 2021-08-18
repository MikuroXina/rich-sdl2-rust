#![allow(unused)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

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
