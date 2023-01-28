//! Cursor control on a window.

use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::{bind, geo::Point, surface::Surface, Result, Sdl, SdlError};

use super::Window;

/// A kind of the system cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
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
    pub(crate) fn as_raw(self) -> bind::SDL_SystemCursor {
        match self {
            SystemCursorKind::Arrow => bind::SDL_SYSTEM_CURSOR_ARROW,
            SystemCursorKind::IBeam => bind::SDL_SYSTEM_CURSOR_IBEAM,
            SystemCursorKind::Wait => bind::SDL_SYSTEM_CURSOR_WAIT,
            SystemCursorKind::Crosshair => bind::SDL_SYSTEM_CURSOR_CROSSHAIR,
            SystemCursorKind::WaitArrow => bind::SDL_SYSTEM_CURSOR_WAITARROW,
            SystemCursorKind::SizeNwse => bind::SDL_SYSTEM_CURSOR_SIZENWSE,
            SystemCursorKind::SizeNesw => bind::SDL_SYSTEM_CURSOR_SIZENESW,
            SystemCursorKind::SizeWe => bind::SDL_SYSTEM_CURSOR_SIZEWE,
            SystemCursorKind::SizeNs => bind::SDL_SYSTEM_CURSOR_SIZENS,
            SystemCursorKind::SizeAll => bind::SDL_SYSTEM_CURSOR_SIZEALL,
            SystemCursorKind::No => bind::SDL_SYSTEM_CURSOR_NO,
            SystemCursorKind::Hand => bind::SDL_SYSTEM_CURSOR_HAND,
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
        f.debug_struct("Cursor").finish_non_exhaustive()
    }
}

assert_not_impl_all!(Cursor: Send, Sync);

impl<'window> Cursor<'window> {
    /// Constructs a system cursor from `kind`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the feature of cursor is unsupported.
    pub fn system(_: &'window Window, kind: SystemCursorKind) -> Result<Self> {
        let cursor = unsafe { bind::SDL_CreateSystemCursor(kind.as_raw()) };
        let cursor = NonNull::new(cursor).ok_or(SdlError::UnsupportedFeature)?;
        Ok(Self {
            cursor,
            window: PhantomData,
        })
    }

    /// Constructs a colored cursor from surface and hot spot point.
    ///
    /// # Errors
    ///
    /// Returns `Err` if coloring a cursor is unsupported.
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

    /// Constructs a completely customized color from data, mask and hot spot point.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to create a custom cursor.
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
    #[must_use]
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
