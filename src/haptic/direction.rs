//! Defining directions and coordinate systems of the haptic movements.

use crate::bind;

/// A direction and coordinate system of the haptic movements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Direction {
    /// A polar coordinate system.
    Polar {
        /// The direction in degrees times 100. The north is 0, the east is 9000, and so on.
        degree_100: i32,
    },
    /// A cartesian coordinate system.
    Cartesian {
        /// The east is positive direction.
        x: i32,
        /// The south is positive direction.
        y: i32,
        /// Z means the level of power if supported it.
        z: i32,
    },
    /// A spherical coordinate system (3-axis haptic device).
    Spherical {
        /// The direction in degrees times 100, rotating from (1, 0, 0) to (0, 1, 0).
        z_degree_100: i32,
        /// The degree times 100, rotating to (0, 0, 1) after above.
        elevation_degree_100: i32,
    },
}

impl Direction {
    pub(super) fn into_raw(self) -> bind::SDL_HapticDirection {
        match self {
            Direction::Polar { degree_100 } => bind::SDL_HapticDirection {
                type_: bind::SDL_HAPTIC_POLAR as u8,
                dir: [degree_100, 0, 0],
            },
            Direction::Cartesian { x, y, z } => bind::SDL_HapticDirection {
                type_: bind::SDL_HAPTIC_CARTESIAN as u8,
                dir: [x, y, z],
            },
            Direction::Spherical {
                z_degree_100,
                elevation_degree_100,
            } => bind::SDL_HapticDirection {
                type_: bind::SDL_HAPTIC_SPHERICAL as u8,
                dir: [z_degree_100, elevation_degree_100, 0],
            },
        }
    }
}

impl From<bind::SDL_HapticDirection> for Direction {
    fn from(raw: bind::SDL_HapticDirection) -> Self {
        match raw.type_ as u32 {
            bind::SDL_HAPTIC_POLAR => Self::Polar {
                degree_100: raw.dir[0],
            },
            bind::SDL_HAPTIC_CARTESIAN => Self::Cartesian {
                x: raw.dir[0],
                y: raw.dir[0],
                z: raw.dir[0],
            },
            bind::SDL_HAPTIC_SPHERICAL => Self::Spherical {
                z_degree_100: raw.dir[0],
                elevation_degree_100: raw.dir[1],
            },
            _ => unreachable!(),
        }
    }
}
