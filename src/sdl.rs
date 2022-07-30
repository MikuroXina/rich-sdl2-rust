use static_assertions::assert_not_impl_all;
use std::{cell::Cell, ffi::CStr, marker::PhantomData};

use crate::bind;

/// A version for SDL2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SdlVersion {
    /// A major number in the version.
    pub major: u8,
    /// A minor number in the version.
    pub minor: u8,
    /// A patch number in the version.
    pub patch: u8,
}

impl From<bind::SDL_version> for SdlVersion {
    fn from(
        bind::SDL_version {
            major,
            minor,
            patch,
        }: bind::SDL_version,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// A root controller for SDL2. But some sub-feature does not require `Sdl`.
pub struct Sdl {
    _phantom: PhantomData<Cell<u8>>,
}

assert_not_impl_all!(Sdl: Send, Sync);

impl Sdl {
    /// Setup the SDL2 system.
    ///
    /// # Panics
    ///
    /// Panics if SDL2 system is already initialized such as there are existing `Sdl` instances
    #[must_use]
    pub fn new() -> Self {
        let ret = unsafe {
            bind::SDL_SetMainReady();
            bind::SDL_Init(0)
        };
        if ret != 0 {
            Self::error_then_panic("Sdl")
        }
        Self {
            _phantom: PhantomData,
        }
    }

    /// Returns the version of SDL2.
    #[must_use]
    pub fn version() -> SdlVersion {
        use bind::SDL_version;
        let mut ver = SDL_version {
            major: 0,
            minor: 0,
            patch: 0,
        };
        unsafe { bind::SDL_GetVersion(&mut ver) }
        ver.into()
    }

    /// Returns the revision string.
    #[must_use]
    pub fn revision_str() -> &'static str {
        let raw_str = unsafe { bind::SDL_GetRevision() };
        unsafe { std::ffi::CStr::from_ptr(raw_str) }
            .to_str()
            .expect("Getting revision failed")
    }

    /// Returns the revision number.
    #[must_use]
    pub fn revision_num() -> u32 {
        (unsafe { bind::SDL_GetRevisionNumber() }) as u32
    }

    /// Gets the platform string.
    #[must_use]
    pub fn platform() -> &'static str {
        let cstr = unsafe { CStr::from_ptr(bind::SDL_GetPlatform()) };
        cstr.to_str().unwrap()
    }

    /// On an unrecoverable error detected, panics with the provided `context`.
    pub fn error_then_panic(context: &'static str) -> ! {
        eprintln!("{} error: {}", context, Self::error());
        panic!("Unrecoverable Sdl error occurred");
    }

    /// Reads the error string from SDL2.
    #[must_use]
    pub fn error() -> String {
        let raw_str = unsafe { bind::SDL_GetError() };
        let error = unsafe { std::ffi::CStr::from_ptr(raw_str) }
            .to_str()
            .expect("Getting error failed")
            .to_owned();
        unsafe { bind::SDL_ClearError() }
        error
    }
}

impl Default for Sdl {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Sdl {
    fn drop(&mut self) {
        unsafe { bind::SDL_Quit() }
    }
}
