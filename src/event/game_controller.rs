use std::ptr::NonNull;

use crate::bind;

pub struct GameController {
    controls: Vec<NonNull<bind::SDL_GameController>>,
}

impl GameController {
    pub fn new() -> Self {
        let num_controls = unsafe {
            bind::SDL_InitSubSystem(bind::SDL_INIT_JOYSTICK);
            bind::SDL_NumJoysticks()
        };
        let controls = (0..num_controls)
            .filter(|&index| unsafe { bind::SDL_IsGameController(index) != 0 })
            .filter_map(|index| {
                let raw = unsafe { bind::SDL_GameControllerOpen(index) };
                NonNull::new(raw)
            })
            .collect();
        Self { controls }
    }
}

impl Drop for GameController {
    fn drop(&mut self) {
        for control in &mut self.controls {
            unsafe { bind::SDL_GameControllerClose(control.as_ptr()) }
        }
        unsafe { bind::SDL_QuitSubSystem(bind::SDL_INIT_JOYSTICK) }
    }
}
