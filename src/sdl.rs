use crate::bind;

pub struct SdlVersion {
    pub major: u8,
    pub minor: u8,
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

pub struct Sdl {}

impl Sdl {
    pub fn new() -> Self {
        let ret = unsafe {
            bind::SDL_SetMainReady();
            bind::SDL_Init(0)
        };
        if ret != 0 {
            Self::error_then_panic("Sdl")
        }
        Self {}
    }

    pub fn version() -> SdlVersion {
        use bind::SDL_version;
        let mut ver = SDL_version {
            major: 0,
            minor: 0,
            patch: 0,
        };
        unsafe { bind::SDL_GetVersion(&mut ver as *mut _) }
        ver.into()
    }

    pub fn revision_str() -> &'static str {
        let raw_str = unsafe { bind::SDL_GetRevision() };
        unsafe { std::ffi::CStr::from_ptr(raw_str) }
            .to_str()
            .expect("Getting revision failed")
    }

    pub fn revision_num() -> u32 {
        (unsafe { bind::SDL_GetRevisionNumber() }) as u32
    }

    pub fn error_then_panic(context: &'static str) -> ! {
        let raw_str = unsafe { bind::SDL_GetError() };
        let error = unsafe { std::ffi::CStr::from_ptr(raw_str) }
            .to_str()
            .expect("Getting error failed")
            .to_owned();
        unsafe { bind::SDL_ClearError() }
        eprintln!("{} error: {}", context, error);
        panic!("Unrecoverable Sdl error occurred");
    }
}

impl Drop for Sdl {
    fn drop(&mut self) {
        unsafe { bind::SDL_Quit() }
    }
}
