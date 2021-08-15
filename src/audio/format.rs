//! Type definitions for a format of audio data.

use bitflags::bitflags;

use crate::bind;

bitflags! {
    /// A flag to represent what type is used in audio data.
    pub struct AudioFormatFlag: u8 {
        /// Whether a type is floating-point number.
        const FLOAT = 1 << 0;
        /// Whether a type is big endian.
        const BIG_ENDIAN = 1 << 4;
        /// Whether a type is signed.
        const SIGNED = 1 << 7;
    }
}

/// A format to represent how is stored samples in audio data.
#[derive(Debug, Clone, Copy)]
pub struct AudioFormat {
    /// A flag to represent characteristics of a type.
    pub flag: AudioFormatFlag,
    /// A size in bits of a type.
    pub bit_size: u8,
}

impl AudioFormat {
    /// A format for signed 8-bit data.
    pub fn signed8() -> Self {
        Self {
            flag: AudioFormatFlag::SIGNED,
            bit_size: 8,
        }
    }

    /// A format for unsigned 8-bit data.
    pub fn unsigned8() -> Self {
        Self {
            flag: AudioFormatFlag::empty(),
            bit_size: 8,
        }
    }

    /// A format for signed little endian 16-bit data.
    pub fn signed16_lsb() -> Self {
        Self {
            flag: AudioFormatFlag::SIGNED,
            bit_size: 16,
        }
    }

    /// A format for unsigned little endian 16-bit data.
    pub fn unsigned16_lsb() -> Self {
        Self {
            flag: AudioFormatFlag::empty(),
            bit_size: 16,
        }
    }

    /// A format for signed big endian 16-bit data.
    pub fn signed16_msb() -> Self {
        Self {
            flag: AudioFormatFlag::BIG_ENDIAN | AudioFormatFlag::SIGNED,
            bit_size: 16,
        }
    }

    /// A format for unsigned big endian 16-bit data.
    pub fn unsigned16_msb() -> Self {
        Self {
            flag: AudioFormatFlag::BIG_ENDIAN,
            bit_size: 16,
        }
    }

    /// A format for signed little endian 32-bit data.
    pub fn signed32_lsb() -> Self {
        Self {
            flag: AudioFormatFlag::SIGNED,
            bit_size: 32,
        }
    }

    /// A format for signed big endian 32-bit data.
    pub fn signed32_msb() -> Self {
        Self {
            flag: AudioFormatFlag::BIG_ENDIAN | AudioFormatFlag::SIGNED,
            bit_size: 32,
        }
    }

    /// A format for floating-point little endian 32-bit data.
    pub fn float32_lsb() -> Self {
        Self {
            flag: AudioFormatFlag::FLOAT | AudioFormatFlag::SIGNED,
            bit_size: 32,
        }
    }

    /// A format for floating-point big endian 32-bit data.
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
