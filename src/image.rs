//! The sdl_image 2.0 wrapper.

use crate::{bind, Result, SdlError, SdlVersion};
use bitflags::bitflags;
use static_assertions::assert_not_impl_all;
use std::{cell::Cell, marker::PhantomData};

pub mod format;
pub mod surface;

bitflags! {
    /// A flag to init an [`Img`] controller.
    pub struct ImgInitFlag: u8 {
        /// To load JPEG format loader.
        const JPEG = 1 << 0;
        /// To load PNG format loader.
        const PNG = 1 << 1;
        /// To load TIFF format loader.
        const TIFF = 1 << 2;
        /// To load WebP format loader.
        const WEBP = 1 << 3;
    }
}

/// A root controller for sdl_image.
pub struct Img {
    _phantom: PhantomData<Cell<u8>>,
}

assert_not_impl_all!(Img: Send, Sync);

impl Img {
    /// Constructs a new root controller.
    pub fn new(flags: ImgInitFlag) -> Result<Self> {
        let ret = unsafe { bind::IMG_Init(flags.bits() as _) };
        if ret != flags.bits() as _ {
            Err(SdlError::UnsupportedFeature)
        } else {
            Ok(Self {
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the library version of SDL2_image.
    pub fn version() -> SdlVersion {
        let raw = unsafe { &*bind::IMG_Linked_Version() };
        SdlVersion {
            major: raw.major,
            minor: raw.minor,
            patch: raw.patch,
        }
    }
}

impl Drop for Img {
    fn drop(&mut self) {
        unsafe { bind::IMG_Quit() }
    }
}
