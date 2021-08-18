//! Querying display information.

use static_assertions::assert_not_impl_all;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::geo::Rect;
use crate::{bind, Video};

pub use self::mode::*;

mod mode;

/// Densities per inch of the display.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dpi {
    /// A diagonal density per inch.
    pub ddpi: f32,
    /// A horizontal density per inch.
    pub hdpi: f32,
    /// A vertical density per inch.
    pub vdpi: f32,
}

/// A display queries bounds, name, dpi and modes.
pub struct Display<'video> {
    index: i32,
    _phantom: PhantomData<&'video Video<'video>>,
}

impl std::fmt::Debug for Display<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Display")
            .field("index", &self.index)
            .finish()
    }
}

assert_not_impl_all!(Display: Send, Sync);

impl<'video> Display<'video> {
    pub(super) fn new(index: i32, _: &'video Video) -> Self {
        Self {
            index,
            _phantom: PhantomData,
        }
    }

    /// Returns the bounds of the display if available.
    pub fn bounds(&self) -> Option<Rect> {
        let mut raw_rect = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetDisplayBounds(self.index, raw_rect.as_mut_ptr()) };
        (ret == 0).then(|| unsafe { raw_rect.assume_init() }.into())
    }

    /// Returns the usable bounds of the display if available.
    pub fn usable_bounds(&self) -> Option<Rect> {
        let mut raw_rect = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetDisplayUsableBounds(self.index, raw_rect.as_mut_ptr()) };
        (ret == 0).then(|| unsafe { raw_rect.assume_init() }.into())
    }

    /// Returns the dpi information [`Dpi`] if available.
    pub fn dpi(&self) -> Option<Dpi> {
        let mut dpi = Dpi {
            ddpi: 0.0,
            hdpi: 0.0,
            vdpi: 0.0,
        };
        let ret = unsafe {
            bind::SDL_GetDisplayDPI(
                self.index,
                &mut dpi.ddpi as *mut _,
                &mut dpi.hdpi as *mut _,
                &mut dpi.vdpi as *mut _,
            )
        };

        (ret == 0).then(|| dpi)
    }

    /// Returns the name of the display.
    pub fn name(&self) -> &str {
        let raw_str = unsafe { bind::SDL_GetDisplayName(self.index) };
        unsafe { CStr::from_ptr(raw_str) }
            .to_str()
            .expect("the display name was not utf8")
    }

    /// Returns the modes supported by the display.
    pub fn modes(&self) -> Vec<Mode> {
        let ret = unsafe { bind::SDL_GetNumDisplayModes(self.index) };
        if ret < 0 {
            return vec![];
        }
        (0..ret)
            .map(|idx| {
                let mut raw = MaybeUninit::uninit();
                let ret = unsafe { bind::SDL_GetDisplayMode(self.index, idx, raw.as_mut_ptr()) };
                debug_assert!(ret != 0);
                let mode = unsafe { raw.assume_init() };
                Mode::new(mode)
            })
            .collect()
    }

    /// Returns the current mode of the display.
    pub fn current_mode(&self) -> Mode {
        let mut raw = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetCurrentDisplayMode(self.index, raw.as_mut_ptr()) };
        debug_assert!(ret != 0);
        let mode = unsafe { raw.assume_init() };
        Mode::new(mode)
    }

    /// Returns the original mode of the display.
    pub fn original_mode(&self) -> Mode {
        let mut raw = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetDesktopDisplayMode(self.index, raw.as_mut_ptr()) };
        debug_assert!(ret != 0);
        let mode = unsafe { raw.assume_init() };
        Mode::new(mode)
    }
}
