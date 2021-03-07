#[derive(Debug, Clone)]
pub enum MouseState {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

impl MouseState {
    pub(crate) fn from_bits(bits: u8) -> Self {
        use MouseState::*;
        match bits {
            1 => Left,
            2 => Middle,
            3 => Right,
            4 => X1,
            5 => X2,
            _ => unreachable!(),
        }
    }
}
