//! Cursor control on a window.

use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::{bind, geo::Point, surface::Surface, Result, Sdl, SdlError};

use super::Window;

/// A kind of the system cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemCursorKind {
    /// The arrow cursor.
    Arrow,
    /// The I beam cursor.
    IBeam,
    /// The waiting cursor.
    Wait,
    /// The crosshair cursor.
    Crosshair,
    /// The waiting cursor with arrow.
    WaitArrow,
    /// The resizing cursor between north west and south east.
    SizeNwse,
    /// The resizing cursor between north east and south west.
    SizeNesw,
    /// The resizing cursor between east and west.
    SizeWe,
    /// The resizing cursor between north and south.
    SizeNs,
    /// The resizing cursor for all directions.
    SizeAll,
    /// The prohibiting cursor.
    No,
    /// The hand cursor.
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

/// A system cursor controller.
pub struct Cursor<'window> {
    cursor: NonNull<bind::SDL_Cursor>,
    window: PhantomData<&'window ()>,
}

impl std::fmt::Debug for Cursor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cursor").finish()
    }
}

assert_not_impl_all!(Cursor: Send, Sync);

impl<'window> Cursor<'window> {
    /// Constructs a system cursor from kind, or `Err` on failure.
    pub fn system(_: &'window Window, kind: SystemCursorKind) -> Result<Self> {
        let cursor = unsafe { bind::SDL_CreateSystemCursor(kind.as_raw()) };
        let cursor = NonNull::new(cursor).ok_or(SdlError::UnsupportedFeature)?;
        Ok(Self {
            cursor,
            window: PhantomData,
        })
    }

    /// Constructs a colored cursor from surface and hot spot point, or `Err` on failure.
    pub fn colored(_: &'window Window, surface: &impl Surface, hot_spot: Point) -> Result<Self> {
        let cursor = unsafe {
            bind::SDL_CreateColorCursor(surface.as_ptr().as_ptr(), hot_spot.x, hot_spot.y)
        };
        let cursor = NonNull::new(cursor).ok_or_else(|| SdlError::Others { msg: Sdl::error() })?;
        Ok(Self {
            cursor,
            window: PhantomData,
        })
    }

    /// Constructs a completely customized color from data, mask and hot spot point, or `Err` on failure.
    pub fn customized(
        _: &'window Window,
        data: &[u8],
        mask: &[u8],
        hot_spot: Point,
    ) -> Result<Self> {
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

    /// Constructs a default cursor, or `None` if unavailable.
    pub fn default(_: &'window Window) -> Option<Self> {
        NonNull::new(unsafe { bind::SDL_GetDefaultCursor() }).map(|cursor| Self {
            cursor,
            window: PhantomData,
        })
    }

    /// Sets the cursor to the current.
    pub fn set(&self) {
        unsafe { bind::SDL_SetCursor(self.cursor.as_ptr()) }
    }

    /// Redraws a cursor.
    pub fn redraw(&self) {
        unsafe { bind::SDL_SetCursor(std::ptr::null_mut()) }
    }
}

impl Drop for Cursor<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeCursor(self.cursor.as_ptr()) }
    }
}
