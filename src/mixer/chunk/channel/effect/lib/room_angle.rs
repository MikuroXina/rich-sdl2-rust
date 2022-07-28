#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct RoomAngle(pub(super) i16);

impl RoomAngle {
    pub(super) fn new(channels: u32, angle: i16) -> Self {
        Self(match channels {
            2 => {
                if 180 < angle {
                    // exchange left and right channels
                    180
                } else {
                    0
                }
            }
            4 | 6 => {
                if 315 < angle {
                    0
                } else if 225 < angle {
                    270
                } else if 135 < angle {
                    180
                } else if 45 < angle {
                    90
                } else {
                    0
                }
            }
            _ => 0,
        })
    }
}
