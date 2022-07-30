use std::ops;

use super::room_angle::RoomAngle;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(super) struct Gain(f64, u8);

impl Default for Gain {
    fn default() -> Self {
        Self(1.0, 255)
    }
}

impl From<u8> for Gain {
    fn from(gain: u8) -> Self {
        Self(gain as f64 / 255.0, gain)
    }
}

impl ops::Mul<Gain> for f64 {
    type Output = f64;

    fn mul(self, rhs: Gain) -> Self::Output {
        self * rhs.0
    }
}

impl ops::MulAssign<Gain> for f64 {
    fn mul_assign(&mut self, rhs: Gain) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Gain> for u8 {
    type Output = u8;

    fn mul(self, rhs: Gain) -> Self::Output {
        self * rhs.1
    }
}

impl ops::MulAssign<Gain> for u8 {
    fn mul_assign(&mut self, rhs: Gain) {
        *self = *self * rhs;
    }
}

pub(super) fn gains(channels: u32, angle: i16, RoomAngle(room_angle): RoomAngle) -> [Gain; 6] {
    let mut left = 255;
    let mut right = 255;
    let mut left_rear = 255;
    let mut right_rear = 255;
    let mut center = 255;
    let back_amp = |angle: i16| (255.0 * angle as f64 / 89.0) as u8;
    let side_amp = |angle: i16| (255.0 * angle as f64 / 179.0) as u8;
    match channels {
        2 => {
            if angle < 90 {
                left = 255 - back_amp(angle);
            } else if angle < 180 {
                left = back_amp(angle - 90);
            } else if angle < 270 {
                right = 255 - back_amp(angle - 180);
            } else {
                right = back_amp(angle - 270);
            }
        }
        4 | 6 => {
            if angle < 45 {
                left = side_amp(180 - angle);
                left_rear = 255 - back_amp(angle + 45);
                right_rear = 255 - side_amp(90 - angle);
            } else if angle < 90 {
                center = side_amp(225 - angle);
                left = side_amp(180 - angle);
                left_rear = 255 - back_amp(135 - angle);
                right_rear = side_amp(90 + angle);
            } else if angle < 135 {
                center = side_amp(225 - angle);
                left = 255 - back_amp(angle - 45);
                right = side_amp(270 - angle);
                left_rear = side_amp(angle);
            } else if angle < 180 {
                center = 255 - back_amp(angle - 90);
                left = 255 - back_amp(225 - angle);
                right = side_amp(270 - angle);
                left_rear = side_amp(angle);
            } else if angle < 225 {
                center = 255 - back_amp(270 - angle);
                left = side_amp(angle - 90);
                right = 255 - back_amp(angle - 135);
                right_rear = side_amp(360 - angle);
            } else if angle < 270 {
                center = side_amp(angle - 135);
                left = side_amp(angle - 90);
                right = 255 - back_amp(315 - angle);
                right_rear = side_amp(360 - angle);
            } else if angle < 315 {
                center = side_amp(angle - 135);
                right = side_amp(angle - 180);
                left_rear = side_amp(450 - angle);
                right_rear = 255 - back_amp(angle - 225);
            } else {
                right = side_amp(angle - 180);
                left_rear = side_amp(450 - angle);
                right_rear = 255 - back_amp(45 - angle);
            }
        }
        _ => {}
    }
    let amps = match room_angle {
        90 => [left_rear, left, right_rear, right],
        180 => {
            if channels == 2 {
                [right, left, 0, 0]
            } else {
                [right_rear, left_rear, right, left]
            }
        }
        270 => [right, right_rear, left, left_rear],
        _ => [left, right, left_rear, right_rear],
    };
    [
        amps[0].into(),
        amps[1].into(),
        amps[2].into(),
        amps[3].into(),
        center.into(),
        255.into(),
    ]
}
