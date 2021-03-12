use std::os::raw::c_int;

use crate::bind;
use crate::geo::Point;

use super::Joystick;

pub struct Trackball<'joystick> {
    index: c_int,
    joystick: &'joystick Joystick,
}

impl<'joystick> Trackball<'joystick> {
    pub fn delta(&self) -> Point {
        let (mut dx, mut dy): (c_int, c_int) = (0, 0);
        let ret = unsafe {
            bind::SDL_JoystickGetBall(
                self.joystick.ptr.as_ptr(),
                self.index,
                &mut dx as *mut _,
                &mut dy as *mut _,
            )
        };
        debug_assert_eq!(ret, 0);
        Point { x: dx, y: dy }
    }
}

pub struct Trackballs<'joystick>(pub Vec<Trackball<'joystick>>);

impl<'joystick> Trackballs<'joystick> {
    pub(super) fn new(joystick: &'joystick Joystick) -> Self {
        let num_balls = unsafe { bind::SDL_JoystickNumBalls(joystick.ptr.as_ptr()) };
        let balls = (0..num_balls)
            .map(|index| Trackball { index, joystick })
            .collect();
        Self(balls)
    }
}
