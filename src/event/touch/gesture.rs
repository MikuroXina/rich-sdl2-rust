use std::marker::PhantomData;

use crate::{bind, file::RwOps, Result, Sdl, SdlError};

use super::TouchDevice;

#[derive(Debug, Clone)]
pub struct Gesture(bind::SDL_GestureID);

impl Gesture {
    pub fn save_dollar_template_all(dst: &RwOps) -> Result<usize> {
        let ret = unsafe { bind::SDL_SaveAllDollarTemplates(dst.ptr().as_ptr()) };
        if ret == 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as usize)
        }
    }

    pub fn save_dollar_template(&self, dst: &RwOps) -> Result<usize> {
        let ret = unsafe { bind::SDL_SaveDollarTemplate(self.0, dst.ptr().as_ptr()) };
        if ret == 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(ret as usize)
        }
    }
}

#[derive(Debug, Clone)]
pub enum GestureEvent {
    Multi {
        timestamp: u32,
        delta_theta: f32,
        delta_dist: f32,
        x: f32,
        y: f32,
        num_fingers: u16,
    },
    Dollar {
        timestamp: u32,
        touch: TouchDevice,
        gesture: Gesture,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32,
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
        Self::Dollar {
            timestamp: raw.timestamp,
            touch: TouchDevice(raw.touchId, PhantomData),
            gesture: Gesture(raw.gestureId),
            num_fingers: raw.numFingers,
            error: raw.error,
            x: raw.x,
            y: raw.y,
        }
    }
}
