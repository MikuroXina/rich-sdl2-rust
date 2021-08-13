use crate::bind;

pub struct TouchDevice(i64);

impl TouchDevice {
    pub fn all_devices() -> Vec<Self> {
        let num = unsafe { bind::SDL_GetNumTouchDevices() };
        (0..num)
            .map(|index| {
                let id = unsafe { bind::SDL_GetTouchDevice(index) };
                Self(id)
            })
            .collect()
    }

    pub fn record(&self) -> bool {
        unsafe { bind::SDL_RecordGesture(self.0) == 1 }
    }
}
