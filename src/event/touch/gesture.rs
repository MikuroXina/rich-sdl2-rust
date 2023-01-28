//! Gesture controller and events.

use std::marker::PhantomData;

use crate::{bind, file::RwOps, EnumInt, Result, Sdl, SdlError};

use super::TouchDevice;

/// A gesture controller by $1 gesture recognition system.
#[derive(Debug, Clone)]
pub struct Gesture(bind::SDL_GestureID);

impl Gesture {
    /// Saves all gestures into `dst` and returns the numbers of succeed to write.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to save all the template data.
    pub fn save_dollar_template_all(dst: &RwOps) -> Result<usize> {
        let ret = unsafe { bind::SDL_SaveAllDollarTemplates(dst.ptr().as_ptr()) };
        if ret == 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as usize)
        }
    }

    /// Saves gesture into `dst` and returns the numbers of succeed to write.
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to save a template data.
    pub fn save_dollar_template(&self, dst: &RwOps) -> Result<usize> {
        let ret = unsafe { bind::SDL_SaveDollarTemplate(self.0, dst.ptr().as_ptr()) };
        if ret == 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as usize)
        }
    }
}

/// An event on recognized a gesture
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum GestureEvent {
    /// The gesture was multi touches.
    Multi {
        /// When this event occurred.
        timestamp: u32,
        /// The rotating amount of fingers on the gesture.
        delta_theta: f32,
        /// The distance difference among fingers on the gesture.
        delta_dist: f32,
        /// The normalized x coord of the center on the gesture.
        x: f32,
        /// The normalized y coord of the center on the gesture.
        y: f32,
        /// The numbers of fingers on the gesture.
        num_fingers: u16,
    },
    /// The gesture was recognized by $1.
    Dollar {
        /// When this event occurred.
        timestamp: u32,
        /// The touch device the gesture detected.
        touch: TouchDevice,
        /// The id of the nearest gesture.
        gesture: Gesture,
        /// The numbers of fingers on the gesture.
        num_fingers: u32,
        /// The error from the template.
        error: f32,
        /// The normalized x coord of the center on the gesture.
        x: f32,
        /// The normalized y coord of the center on the gesture.
        y: f32,
    },
    /// The gesture was recorded for $1.
    DollarRecord {
        /// The touch device the gesture detected.
        touch: TouchDevice,
        /// The id of the nearest gesture.
        gesture: Gesture,
    },
}

impl From<bind::SDL_MultiGestureEvent> for GestureEvent {
    fn from(raw: bind::SDL_MultiGestureEvent) -> Self {
        Self::Multi {
            timestamp: raw.timestamp,
            delta_theta: raw.dTheta,
            delta_dist: raw.dDist,
            x: raw.x,
            y: raw.y,
            num_fingers: raw.numFingers,
        }
    }
}

impl From<bind::SDL_DollarGestureEvent> for GestureEvent {
    fn from(raw: bind::SDL_DollarGestureEvent) -> Self {
        match raw.type_ as EnumInt {
            bind::SDL_DOLLARGESTURE => Self::Dollar {
                timestamp: raw.timestamp,
                touch: TouchDevice(raw.touchId, PhantomData),
                gesture: Gesture(raw.gestureId),
                num_fingers: raw.numFingers,
                error: raw.error,
                x: raw.x,
                y: raw.y,
            },
            bind::SDL_DOLLARRECORD => Self::DollarRecord {
                touch: TouchDevice(raw.touchId, PhantomData),
                gesture: Gesture(raw.gestureId),
            },
            _ => unreachable!(),
        }
    }
}
