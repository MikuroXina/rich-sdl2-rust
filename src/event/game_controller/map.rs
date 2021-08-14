use crate::bind;

#[derive(Debug)]
pub enum MapInput {
    Button { index: u32 },
    Axis { index: u32 },
    Hat { index: u32, mask: u32 },
}

impl From<bind::SDL_GameControllerButtonBind> for MapInput {
    fn from(raw: bind::SDL_GameControllerButtonBind) -> Self {
        use MapInput::*;
        match raw.bindType {
            bind::SDL_GameControllerBindType_SDL_CONTROLLER_BINDTYPE_BUTTON => Button {
                index: unsafe { raw.value.button } as u32,
            },
            bind::SDL_GameControllerBindType_SDL_CONTROLLER_BINDTYPE_AXIS => Axis {
                index: unsafe { raw.value.axis } as u32,
            },
            bind::SDL_GameControllerBindType_SDL_CONTROLLER_BINDTYPE_HAT => Hat {
                index: unsafe { raw.value.hat.hat } as u32,
                mask: unsafe { raw.value.hat.hat_mask } as u32,
            },
            _ => unreachable!(), // NONE does not occur on this wrapper
        }
    }
}
