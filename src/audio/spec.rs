use std::sync::{Arc, Mutex};

use super::format::AudioFormat;

#[derive(Debug)]
pub struct AudioSpecBuilder {
    sample_freq: u32,
    format: AudioFormat,
    channels: u8,
    samples: u16,
}

impl Default for AudioSpecBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioSpecBuilder {
    pub fn new() -> Self {
        Self {
            sample_freq: 44100,
            format: AudioFormat::signed32_lsb(),
            channels: 2,
            samples: 4096,
        }
    }

    pub fn sample_freq(&mut self, value: u32) -> &mut Self {
        self.sample_freq = value;
        self
    }

    pub fn format(&mut self, value: AudioFormat) -> &mut Self {
        self.format = value;
        self
    }

    pub fn channels(&mut self, value: u8) -> &mut Self {
        self.channels = value;
        self
    }

    pub fn samples(&mut self, value: u16) -> &mut Self {
        self.samples = value;
        self
    }

    pub fn build<T>(self, callback: AudioCallback<T>, user_data: Arc<Mutex<T>>) -> AudioSpec<T> {
        AudioSpec {
            sample_freq: self.sample_freq,
            format: self.format,
            channels: self.channels,
            silence: 0,
            samples: self.samples,
            size: 0,
            callback,
            user_data,
        }
    }
}

type AudioCallback<T> = fn(Arc<Mutex<T>>, &mut [u8]);

pub struct AudioSpec<T> {
    pub sample_freq: u32,
    pub format: AudioFormat,
    pub channels: u8,
    pub silence: u8,
    pub samples: u16,
    pub size: u32,
    pub callback: AudioCallback<T>,
    pub user_data: Arc<Mutex<T>>,
}
