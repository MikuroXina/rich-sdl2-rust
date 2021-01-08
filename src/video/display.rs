use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::geo::Rect;
use crate::{bind, Video};

use self::mode::Mode;

pub mod mode;
pub mod pixel_format;

pub struct Dpi {
    pub ddpi: f32,
    pub hdpi: f32,
    pub vdpi: f32,
}

pub struct Display<'video> {
    index: i32,
    _phantom: PhantomData<&'video ()>,
}

impl<'video> Display<'video> {
    pub(super) fn new(index: i32, _: &'video Video) -> Self {
        Self {
            index,
            _phantom: PhantomData,
        }
    }

    pub fn bounds(&self) -> Option<Rect> {
        let mut raw_rect = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetDisplayBounds(self.index, raw_rect.as_mut_ptr()) };
        if ret != 0 {
            None
        } else {
            Some(unsafe { raw_rect.assume_init() }.into())
        }
    }

    pub fn usable_bounds(&self) -> Option<Rect> {
        let mut raw_rect = MaybeUninit::uninit();
        let ret = unsafe { bind::SDL_GetDisplayUsableBounds(self.index, raw_rect.as_mut_ptr()) };
        if ret != 0 {
            None
        } else {
            Some(unsafe { raw_rect.assume_init() }.into())
        }
    }

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
        if ret != 0 {
            None
        } else {
            Some(dpi)
        }
    }

    pub fn name(&self) -> &str {
        let raw_str = unsafe { bind::SDL_GetDisplayName(self.index) };
        unsafe { CStr::from_ptr(raw_str) }
            .to_str()
            .expect("Getting display name failed")
    }

    pub fn modes(&self) -> Vec<Mode> {
        let ret = unsafe { bind::SDL_GetNumDisplayModes(self.index) };
        if ret < 0 {
            return vec![];
        }
        (0..ret).map(|idx| Mode::new(idx, &self)).collect()
    }

    // TODO(MikuroXina): get current and original display mode
}
