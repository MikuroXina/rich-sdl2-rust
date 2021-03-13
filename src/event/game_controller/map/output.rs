use crate::event::joystick::{axis::Axis, button::Button};

pub enum MapOutput<'joystick> {
    Button(Button<'joystick>),
    Axis {
        axis: Axis<'joystick>,
        min: i16,
        max: i16,
    },
}
