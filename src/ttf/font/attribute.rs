use std::{borrow::Cow, ffi::CStr};

use super::Font;
use crate::bind;

/// An extension for getting the attributes of the fonts.
pub trait AttributeExt {
    /// Returns the numbers of the faces, the sub-font in the font.
    fn faces(&self) -> usize;
    /// Returns whether the font is fixed width.
    fn is_fixed_width(&self) -> bool;
    /// Returns the family name of the font.
    fn family_name(&self) -> Option<Cow<str>>;
    /// Returns the style name of the font.
    fn style_name(&self) -> Option<Cow<str>>;
}

impl AttributeExt for Font<'_> {
    fn faces(&self) -> usize {
        unsafe { bind::TTF_FontFaces(self.ptr.as_ptr()) as _ }
    }

    fn is_fixed_width(&self) -> bool {
        unsafe { bind::TTF_FontFaceIsFixedWidth(self.ptr.as_ptr()) != 0 }
    }

    fn family_name(&self) -> Option<Cow<str>> {
        let ptr = unsafe { bind::TTF_FontFaceFamilyName(self.ptr.as_ptr()) };
        (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_string_lossy())
    }

    fn style_name(&self) -> Option<Cow<str>> {
        let ptr = unsafe { bind::TTF_FontFaceStyleName(self.ptr.as_ptr()) };
        (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_string_lossy())
    }
}
