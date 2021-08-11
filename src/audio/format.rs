use bitflags::bitflags;

use crate::bind;

bitflags! {
    pub struct AudioFormatFlag: u8 {
        const FLOAT = 1 << 0;
        const BIG_ENDIAN = 1 << 4;
        const SIGNED = 1 << 7;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioFormat {
    pub flag: AudioFormatFlag,
    pub bit_size: u8,
}

impl AudioFormat {
    pub fn signed8() -> Self {
        Self {
            flag: AudioFormatFlag::SIGNED,
            bit_size: 8,
        }
    }

    pub fn unsigned8() -> Self {
        Self {
            flag: AudioFormatFlag::empty(),
            bit_size: 8,
        }
    }

    pub fn signed16_lsb() -> Self {
        Self {
            flag: AudioFormatFlag::SIGNED,
            bit_size: 16,
        }
    }

    pub fn unsigned16_lsb() -> Self {
        Self {
            flag: AudioFormatFlag::empty(),
            bit_size: 16,
        }
    }

    pub fn signed16_msb() -> Self {
        Self {
            flag: AudioFormatFlag::BIG_ENDIAN | AudioFormatFlag::SIGNED,
            bit_size: 16,
        }
    }

    pub fn unsigned16_msb() -> Self {
        Self {
            flag: AudioFormatFlag::BIG_ENDIAN,
            bit_size: 16,
        }
    }

    pub fn signed32_lsb() -> Self {
        Self {
            flag: AudioFormatFlag::SIGNED,
            bit_size: 32,
        }
    }

    pub fn signed32_msb() -> Self {
        Self {
            flag: AudioFormatFlag::BIG_ENDIAN | AudioFormatFlag::SIGNED,
            bit_size: 32,
        }
    }

    pub fn float32_lsb() -> Self {
        Self {
            flag: AudioFormatFlag::FLOAT | AudioFormatFlag::SIGNED,
            bit_size: 32,
        }
    }

    pub fn float32_msb() -> Self {
        Self {
            flag: AudioFormatFlag::FLOAT | AudioFormatFlag::BIG_ENDIAN | AudioFormatFlag::SIGNED,
            bit_size: 32,
        }
    }

    pub(super) fn as_raw(self) -> u16 {
        (self.flag.bits as u16) << 8 | self.bit_size as u16
    }
}

impl From<u16> for AudioFormat {
    fn from(raw: u16) -> Self {
        Self {
            flag: AudioFormatFlag::from_bits((raw >> 8) as u8).unwrap(),
            bit_size: (raw & 0xff) as u8,
        }
    }
}
