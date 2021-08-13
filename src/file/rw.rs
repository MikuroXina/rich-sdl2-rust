use std::{
    ffi::CString,
    io::{self, Seek},
    marker::PhantomData,
    os::raw::c_int,
    ptr::NonNull,
};

use super::mode::OpenMode;
use crate::{
    bind::{self, size_t},
    Result, Sdl, SdlError,
};

pub struct RwOps<'a> {
    ptr: NonNull<bind::SDL_RWops>,
    _phantom: PhantomData<&'a mut ()>,
}

impl<'a> RwOps<'a> {
    pub(crate) unsafe fn ptr(&self) -> NonNull<bind::SDL_RWops> {
        self.ptr
    }

    pub fn from_file(file_name: &str, mode: OpenMode) -> Result<RwOps<'static>> {
        let cstr = CString::new(file_name).expect("file_name must not be empty");
        let ptr = unsafe { bind::SDL_RWFromFile(cstr.as_ptr(), mode.into_raw().as_ptr()) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(RwOps {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    pub fn from_mem(buf: &'a [u8]) -> Result<Self> {
        let ptr = unsafe { bind::SDL_RWFromConstMem(buf.as_ptr() as *const _, buf.len() as c_int) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(RwOps {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    pub fn from_mem_mut(buf: &'a mut [u8]) -> Result<Self> {
        let ptr = unsafe { bind::SDL_RWFromMem(buf.as_mut_ptr() as *mut _, buf.len() as c_int) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(RwOps {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    pub fn size(&self) -> Result<usize> {
        let ret = unsafe { bind::SDL_RWsize(self.ptr.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as usize)
        }
    }

    pub fn tell(&mut self) -> io::Result<u64> {
        self.seek(io::SeekFrom::Current(0))
    }

    pub fn read_u8(&mut self) -> u8 {
        unsafe { bind::SDL_ReadU8(self.ptr.as_ptr()) }
    }
    pub fn read_be16(&mut self) -> u16 {
        unsafe { bind::SDL_ReadBE16(self.ptr.as_ptr()) }
    }
    pub fn read_le16(&mut self) -> u16 {
        unsafe { bind::SDL_ReadLE16(self.ptr.as_ptr()) }
    }
    pub fn read_be32(&mut self) -> u32 {
        unsafe { bind::SDL_ReadBE32(self.ptr.as_ptr()) }
    }
    pub fn read_le32(&mut self) -> u32 {
        unsafe { bind::SDL_ReadLE32(self.ptr.as_ptr()) }
    }
    pub fn read_be64(&mut self) -> u64 {
        unsafe { bind::SDL_ReadBE64(self.ptr.as_ptr()) }
    }
    pub fn read_le64(&mut self) -> u64 {
        unsafe { bind::SDL_ReadLE64(self.ptr.as_ptr()) }
    }

    pub fn write_u8(&mut self, value: u8) -> bool {
        unsafe { bind::SDL_WriteU8(self.ptr.as_ptr(), value) == 1 }
    }
    pub fn write_be16(&mut self, value: u16) -> bool {
        unsafe { bind::SDL_WriteBE16(self.ptr.as_ptr(), value) == 1 }
    }
    pub fn write_le16(&mut self, value: u16) -> bool {
        unsafe { bind::SDL_WriteLE16(self.ptr.as_ptr(), value) == 1 }
    }
    pub fn write_be32(&mut self, value: u32) -> bool {
        unsafe { bind::SDL_WriteBE32(self.ptr.as_ptr(), value) == 1 }
    }
    pub fn write_le32(&mut self, value: u32) -> bool {
        unsafe { bind::SDL_WriteLE32(self.ptr.as_ptr(), value) == 1 }
    }
    pub fn write_be64(&mut self, value: u64) -> bool {
        unsafe { bind::SDL_WriteBE64(self.ptr.as_ptr(), value) == 1 }
    }
    pub fn write_le64(&mut self, value: u64) -> bool {
        unsafe { bind::SDL_WriteLE64(self.ptr.as_ptr(), value) == 1 }
    }
}

impl Drop for RwOps<'_> {
    fn drop(&mut self) {
        let _ = unsafe { bind::SDL_RWclose(self.ptr.as_ptr()) };
    }
}

impl io::Read for RwOps<'_> {
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

impl io::Seek for RwOps<'_> {
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

impl io::Write for RwOps<'_> {
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
