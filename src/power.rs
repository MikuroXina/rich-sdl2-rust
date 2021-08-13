use crate::bind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Unknown,
    OnBattery,
    NoBattery,
    Charging,
    Charged,
}

impl From<bind::SDL_PowerState> for PowerState {
    fn from(raw: bind::SDL_PowerState) -> Self {
        use PowerState::*;
        match raw {
            bind::SDL_PowerState_SDL_POWERSTATE_UNKNOWN => Unknown,
            bind::SDL_PowerState_SDL_POWERSTATE_ON_BATTERY => OnBattery,
            bind::SDL_PowerState_SDL_POWERSTATE_NO_BATTERY => NoBattery,
            bind::SDL_PowerState_SDL_POWERSTATE_CHARGING => Charging,
            bind::SDL_PowerState_SDL_POWERSTATE_CHARGED => Charged,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerInfo {
    pub state: PowerState,
    pub remaining_seconds: Option<u32>,
    pub remaining_ratio: Option<u32>,
}

impl PowerInfo {
    pub fn now() -> Self {
        let mut remaining_seconds = 0;
        let mut remaining_ratio = 0;
        let state = unsafe {
            bind::SDL_GetPowerInfo(
                &mut remaining_seconds as *mut _,
                &mut remaining_ratio as *mut _,
            )
        };
        Self {
            state: state.into(),
            remaining_seconds: (0 <= remaining_seconds).then(|| remaining_seconds as _),
            remaining_ratio: (0 <= remaining_ratio).then(|| remaining_ratio as _),
        }
    }
}
