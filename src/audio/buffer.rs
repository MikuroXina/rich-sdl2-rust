use crate::bind;

use super::format::AudioFormat;

pub struct AudioBuffer<T> {
    format: AudioFormat,
    samples: u32,
    channels: u8,
    buffer: Vec<T>,
}

impl<T> AudioBuffer<T> {
    pub fn new(format: AudioFormat, samples: u32, channels: u8, buffer: Vec<T>) -> Self {
        assert_eq!(format.bit_size as usize, std::mem::size_of::<T>() * 8);
        Self {
            format,
            samples,
            channels,
            buffer,
        }
    }
}
