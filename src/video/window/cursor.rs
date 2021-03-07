use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::geo::Point;
use crate::surface::Surface;
use crate::{bind, Sdl, SdlError};

use super::Window;

pub enum SystemCursorKind {
    Arrow,
    IBeam,
    Wait,
    Crosshair,
    WaitArrow,
    SizeNwse,
    SizeNesw,
    SizeWe,
    SizeNs,
    SizeAll,
    No,
    Hand,
}

impl SystemCursorKind {
    pub(crate) fn as_raw(&self) -> bind::SDL_SystemCursor {
        use SystemCursorKind::*;
        match self {
            Arrow => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_ARROW,
            IBeam => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_IBEAM,
            Wait => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_WAIT,
            Crosshair => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_CROSSHAIR,
            WaitArrow => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_WAITARROW,
            SizeNwse => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENWSE,
            SizeNesw => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENESW,
            SizeWe => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEWE,
            SizeNs => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENS,
            SizeAll => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEALL,
            No => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_NO,
            Hand => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_HAND,
        }
    }
}

pub struct Cursor<'window> {
    cursor: NonNull<bind::SDL_Cursor>,
    window: PhantomData<&'window ()>,
}

impl<'window> Cursor<'window> {
    pub fn system(_: &'window Window, kind: SystemCursorKind) -> Result<Self, SdlError> {
        let cursor = unsafe { bind::SDL_CreateSystemCursor(kind.as_raw()) };
        let cursor = NonNull::new(cursor).ok_or_else(|| SdlError::UnsupportedFeature)?;
        Ok(Self {
            cursor,
            window: PhantomData,
        })
    }

    pub fn colored(
        _: &'window Window,
        surface: &impl Surface,
        hot_spot: Point,
    ) -> Result<Self, SdlError> {
        let cursor = unsafe {
            bind::SDL_CreateColorCursor(surface.as_ptr().as_ptr(), hot_spot.x, hot_spot.y)
        };
        let cursor = NonNull::new(cursor).ok_or_else(|| SdlError::Others { msg: Sdl::error() })?;
        Ok(Self {
            cursor,
            window: PhantomData,
        })
    }

    pub fn customized(
        _: &'window Window,
        data: &[u8],
        mask: &[u8],
        hot_spot: Point,
    ) -> Result<Self, SdlError> {
        debug_assert_eq!(data.len(), mask.len());
        let width_height = (data.len() / 4) as i32;
        let cursor = unsafe {
            bind::SDL_CreateCursor(
                data.as_ptr(),
                mask.as_ptr(),
                width_height,
                width_height,
                hot_spot.x,
                hot_spot.y,
            )
        };
        let cursor = NonNull::new(cursor).ok_or_else(|| SdlError::Others { msg: Sdl::error() })?;
        Ok(Self {
            cursor,
            window: PhantomData,
        })
    }

    pub fn default(_: &'window Window) -> Option<Self> {
        NonNull::new(unsafe { bind::SDL_GetDefaultCursor() }).map(|cursor| Self {
            cursor,
            window: PhantomData,
        })
    }

    pub fn set(&self) {
        unsafe { bind::SDL_SetCursor(self.cursor.as_ptr()) }
    }

    pub fn redraw(&self) {
        unsafe { bind::SDL_SetCursor(std::ptr::null_mut()) }
    }
}

impl Drop for Cursor<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeCursor(self.cursor.as_ptr()) }
    }
}
