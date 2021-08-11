use crate::bind;

#[derive(Debug, Clone)]
pub enum Direction {
    Polar {
        degree_100: i32,
    },
    Cartesian {
        x: i32,
        y: i32,
        z: i32,
    },
    Spherical {
        z_degree_100: i32,
        elevation_degree_100: i32,
    },
}

impl Direction {
    pub(super) fn to_raw(&self) -> bind::SDL_HapticDirection {
        match *self {
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
