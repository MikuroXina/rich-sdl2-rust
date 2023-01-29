use std::{
    ffi::CString,
    io::{self, Seek},
    marker::PhantomData,
    os::raw::c_int,
    ptr::NonNull,
};

use super::mode::OpenMode;
use crate::{bind, Result, Sdl, SdlError};

/// A file handler, how to read and write from file on SDL2.
pub struct RwOps<'a> {
    ptr: NonNull<bind::SDL_RWops>,
    _phantom: PhantomData<&'a mut ()>,
}

impl std::fmt::Debug for RwOps<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RwOps")
            .field("size", &self.size())
            .finish_non_exhaustive()
    }
}

impl<'a> RwOps<'a> {
    /// Returns the raw pointer of the file handler.
    ///
    /// # Safety
    ///
    /// Dereferencing the pointer and using the field is not recommended. Please use carefully.
    #[must_use]
    pub unsafe fn ptr(&self) -> NonNull<bind::SDL_RWops> {
        self.ptr
    }

    /// Constructs from file name with the open mode.
    ///
    /// # Panics
    ///
    /// Panics if `file_name` was empty.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to open the file with `mode`.
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

    /// Constructs from the buffer `&[u8]`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to make it a read-only memory buffer.
    pub fn from_mem(buf: &'a [u8]) -> Result<Self> {
        let ptr = unsafe { bind::SDL_RWFromConstMem(buf.as_ptr().cast(), buf.len() as c_int) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(RwOps {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Constructs from the mutable buffer `&mut [u8]`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to make it a read-write memory buffer.
    pub fn from_mem_mut(buf: &'a mut [u8]) -> Result<Self> {
        let ptr = unsafe { bind::SDL_RWFromMem(buf.as_mut_ptr().cast(), buf.len() as c_int) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(RwOps {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the size of the read/write target.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to get the size of buffer.
    pub fn size(&self) -> Result<usize> {
        let ret = unsafe { bind::SDL_RWsize(self.ptr.as_ptr()) };
        if ret < 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as usize)
        }
    }

    /// Returns the current position of seeking.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to seek to get the current position.
    pub fn tell(&mut self) -> io::Result<u64> {
        self.stream_position()
    }

    /// Reads and pops the 8-bits value.
    pub fn read_u8(&mut self) -> u8 {
        unsafe { bind::SDL_ReadU8(self.ptr.as_ptr()) }
    }
    /// Reads and pops the big endian 16-bits value.
    pub fn read_be16(&mut self) -> u16 {
        unsafe { bind::SDL_ReadBE16(self.ptr.as_ptr()) }
    }
    /// Reads and pops the little endian 16-bits value.
    pub fn read_le16(&mut self) -> u16 {
        unsafe { bind::SDL_ReadLE16(self.ptr.as_ptr()) }
    }
    /// Reads and pops the big endian 32-bits value.
    pub fn read_be32(&mut self) -> u32 {
        unsafe { bind::SDL_ReadBE32(self.ptr.as_ptr()) }
    }
    /// Reads and pops the little endian 32-bits value.
    pub fn read_le32(&mut self) -> u32 {
        unsafe { bind::SDL_ReadLE32(self.ptr.as_ptr()) }
    }
    /// Reads and pops the big endian 64-bits value.
    pub fn read_be64(&mut self) -> u64 {
        unsafe { bind::SDL_ReadBE64(self.ptr.as_ptr()) }
    }
    /// Reads and pops the little endian 64-bits value.
    pub fn read_le64(&mut self) -> u64 {
        unsafe { bind::SDL_ReadLE64(self.ptr.as_ptr()) }
    }

    /// Writes the 8-bits value.
    pub fn write_u8(&mut self, value: u8) -> bool {
        unsafe { bind::SDL_WriteU8(self.ptr.as_ptr(), value) == 1 }
    }
    /// Writes the big endian 16-bits value.
    pub fn write_be16(&mut self, value: u16) -> bool {
        unsafe { bind::SDL_WriteBE16(self.ptr.as_ptr(), value) == 1 }
    }
    /// Writes the little endian 16-bits value.
    pub fn write_le16(&mut self, value: u16) -> bool {
        unsafe { bind::SDL_WriteLE16(self.ptr.as_ptr(), value) == 1 }
    }
    /// Writes the big endian 32-bits value.
    pub fn write_be32(&mut self, value: u32) -> bool {
        unsafe { bind::SDL_WriteBE32(self.ptr.as_ptr(), value) == 1 }
    }
    /// Writes the little endian 32-bits value.
    pub fn write_le32(&mut self, value: u32) -> bool {
        unsafe { bind::SDL_WriteLE32(self.ptr.as_ptr(), value) == 1 }
    }
    /// Writes the big endian 64-bits value.
    pub fn write_be64(&mut self, value: u64) -> bool {
        unsafe { bind::SDL_WriteBE64(self.ptr.as_ptr(), value) == 1 }
    }
    /// Writes the little endian 64-bits value.
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
                buf.as_mut_ptr().cast(),
                1,
                buf.len() as _,
            )
        };
        if ret == 0 {
            Err(io::Error::new(
                io::ErrorKind::Other,
                SdlError::Others { msg: Sdl::error() },
            ))
        } else {
            Ok(ret as _)
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
    #[allow(clippy::unnecessary_cast)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written =
            unsafe { bind::SDL_RWwrite(self.ptr.as_ptr(), buf.as_ptr().cast(), 1, buf.len() as _) }
                as usize;
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
