use std::{
    io::{self, Seek},
    ptr::NonNull,
};

use crate::{
    bind::{self, size_t},
    Result, Sdl, SdlError,
};

pub struct RwOps {
    ptr: NonNull<bind::SDL_RWops>,
}

impl RwOps {
    fn size(&self) -> Result<usize> {
        let ret = unsafe { bind::SDL_RWsize(self.ptr.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as usize)
        }
    }

    fn tell(&mut self) -> io::Result<u64> {
        self.seek(io::SeekFrom::Current(0))
    }
}

impl Drop for RwOps {
    fn drop(&mut self) {
        let _ = unsafe { bind::SDL_RWclose(self.ptr.as_ptr()) };
    }
}

impl io::Read for RwOps {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ret = unsafe {
            bind::SDL_RWread(
                self.ptr.as_ptr(),
                buf.as_mut_ptr() as *mut _,
                1,
                buf.len() as size_t,
            )
        };
        if ret == 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(ret as usize)
        }
    }
}

impl io::Seek for RwOps {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let ret = match pos {
            io::SeekFrom::Start(pos) => unsafe {
                bind::SDL_RWseek(self.ptr.as_ptr(), pos as i64, 0)
            },
            io::SeekFrom::End(pos) => unsafe { bind::SDL_RWseek(self.ptr.as_ptr(), pos, 2) },
            io::SeekFrom::Current(pos) => unsafe { bind::SDL_RWseek(self.ptr.as_ptr(), pos, 1) },
        };
        if ret < 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(ret as u64)
        }
    }
}

impl io::Write for RwOps {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = unsafe {
            bind::SDL_RWwrite(
                self.ptr.as_ptr(),
                buf.as_ptr() as *const _,
                1,
                buf.len() as size_t,
            ) as usize
        };
        if written < buf.len() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(written)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
