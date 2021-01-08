use std::ptr::NonNull;

use buffer::BufferSurface;

use crate::pixel_format::PixelFormat;
use crate::{bind, Result, Sdl, SdlError};

pub mod buffer;
pub mod window;

pub trait Surface {
    fn as_ptr(&self) -> *mut bind::SDL_Surface;

    fn pixel_format(&self) -> PixelFormat
    where
        Self: Sized,
    {
        PixelFormat::from_surface(self as &dyn Surface)
    }

    fn convert(&self, format: PixelFormat) -> Result<BufferSurface> {
        let raw = unsafe { bind::SDL_ConvertSurface(self.as_ptr(), &format.raw as *const _, 0) };
        NonNull::new(raw)
            .map(|surface| BufferSurface::new(surface))
            .ok_or_else(|| SdlError::Others { msg: Sdl::error() })
    }
}
