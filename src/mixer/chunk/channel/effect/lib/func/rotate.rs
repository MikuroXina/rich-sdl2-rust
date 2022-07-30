/// A rotator of some speaker channels by the room angle.
#[macro_export(local_inner_macros)]
macro_rules! rotate {
    (2ch, $room_angle:expr, $gained:ident) => {
        if $room_angle == 180 {
            $gained.swap(0, 1);
        }
    };
    (4ch, $room_angle:expr, $gained:ident) => {
        match $room_angle {
            90 => {
                $gained.swap(0, 1);
                $gained.swap(1, 3);
                $gained.swap(2, 3);
            }
            180 => {
                $gained.swap(0, 3);
                $gained.swap(1, 2);
            }
            270 => {
                $gained.swap(0, 1);
                $gained.swap(2, 3);
                $gained.swap(0, 3);
            }
            _ => {}
        }
    };
    (6ch, $room_angle:expr, $gained:ident) => {
        match $room_angle {
            90 => {
                $gained[4] = ($gained[1] + $gained[3]) / 2.0;
                $gained.swap(0, 1);
                $gained.swap(2, 3);
                $gained.swap(1, 2);
            }
            180 => {
                $gained[4] = ($gained[2] + $gained[3]) / 2.0;
                $gained.swap(0, 3);
                $gained.swap(1, 2);
            }
            270 => {
                $gained[4] = ($gained[0] + $gained[2]) / 2.0;
                $gained.swap(0, 1);
                $gained.swap(2, 3);
                $gained.swap(0, 3);
            }
            _ => {}
        }
    };
}
