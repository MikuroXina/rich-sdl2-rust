//! A definition of a mapping structure.

use crate::bind;

/// A mapping that the logical button is attached to the physical index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MapInput {
    /// It is attached to the button.
    Button {
        /// The physical index of the button.
        index: u32,
    },
    /// It is attached to the axis.
    Axis {
        /// The physical index of the axis.
        index: u32,
    },
    /// It is attached to the hat.
    Hat {
        /// The physical index of the hat.
        index: u32,
        /// The mask of the hat.
        mask: u32,
    },
}

impl From<bind::SDL_GameControllerButtonBind> for MapInput {
    fn from(raw: bind::SDL_GameControllerButtonBind) -> Self {
        match raw.bindType {
            bind::SDL_CONTROLLER_BINDTYPE_BUTTON => MapInput::Button {
                index: unsafe { raw.value.button } as u32,
            },
            bind::SDL_CONTROLLER_BINDTYPE_AXIS => MapInput::Axis {
                index: unsafe { raw.value.axis } as u32,
            },
            bind::SDL_CONTROLLER_BINDTYPE_HAT => MapInput::Hat {
                index: unsafe { raw.value.hat.hat } as u32,
                mask: unsafe { raw.value.hat.hat_mask } as u32,
            },
            _ => unreachable!(), // NONE does not occur on this wrapper
        }
    }
}
