use std::marker::PhantomData;

use crate::bind;

use super::TouchDevice;

pub struct Gesture(bind::SDL_GestureID);

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
