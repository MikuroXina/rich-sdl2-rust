//! This module provides wrapper for SDL2_ttf and abstractions of font rendering APIs.

use static_assertions::assert_not_impl_all;
use std::{cell::Cell, marker::PhantomData, os::raw::c_int};

use crate::{bind, Result, Sdl, SdlError, SdlVersion};

pub mod font;
pub mod script;

/// A root SDL2_ttf controller.
#[derive(Debug)]
pub struct Ttf {
    _phantom: PhantomData<Cell<u8>>,
}

assert_not_impl_all!(Ttf: Send, Sync);

impl Ttf {
    /// Constructs a root controller.
    pub fn new() -> Self {
        let ret = unsafe { bind::TTF_Init() };
        if ret == -1 {
            Sdl::error_then_panic("Ttf");
        }
        Self {
            _phantom: PhantomData,
        }
    }

    /// Returns the library version of SDL2_ttf.
    pub fn version() -> SdlVersion {
        let raw = unsafe { &*bind::TTF_Linked_Version() };
        SdlVersion {
            major: raw.major,
            minor: raw.minor,
            patch: raw.patch,
        }
    }
}

impl Drop for Ttf {
    fn drop(&mut self) {
        unsafe { bind::TTF_Quit() }
    }
}

impl Default for Ttf {
    fn default() -> Self {
        Self::new()
    }
}

/// A direction of a text segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// A writing from left to right.
    Ltr,
    /// A writing from right to left.
    Rtl,
    /// A writing from top to bottom.
    Ttb,
    /// A writing from bottom to top.
    Btt,
}

impl Direction {
    fn into_raw(self) -> c_int {
        match self {
            Direction::Ltr => 4,
            Direction::Rtl => 5,
            Direction::Ttb => 6,
            Direction::Btt => 7,
        }
    }

    /// Sets the direction of a text segment.
    pub fn set_direction(self) -> Result<()> {
        let ret = unsafe { bind::TTF_SetDirection(self.into_raw()) };
        if ret != 0 {
            Err(SdlError::UnsupportedFeature)
        } else {
            Ok(())
        }
    }
}
