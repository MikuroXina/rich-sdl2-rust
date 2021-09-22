//! System power monitoring.

use crate::bind;

/// A state of power in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    /// A state cannot be known.
    Unknown,
    /// It has a battery, not charging.
    OnBattery,
    /// It has no batteries.
    NoBattery,
    /// It has a battery, charging.
    Charging,
    /// It has a battery, completed to charge.
    Charged,
}

impl From<bind::SDL_PowerState> for PowerState {
    fn from(raw: bind::SDL_PowerState) -> Self {
        use PowerState::*;
        match raw {
            bind::SDL_POWERSTATE_UNKNOWN => Unknown,
            bind::SDL_POWERSTATE_ON_BATTERY => OnBattery,
            bind::SDL_POWERSTATE_NO_BATTERY => NoBattery,
            bind::SDL_POWERSTATE_CHARGING => Charging,
            bind::SDL_POWERSTATE_CHARGED => Charged,
            _ => unreachable!(),
        }
    }
}

/// A detail information of the system battery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerInfo {
    /// The battery state of the system.
    pub state: PowerState,
    /// The remaining amount of battery in seconds if available.
    pub remaining_seconds: Option<u32>,
    /// The remaining amount of battery in percent if available.
    pub remaining_ratio: Option<u32>,
}

impl PowerInfo {
    /// Returns a power information at now.
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
