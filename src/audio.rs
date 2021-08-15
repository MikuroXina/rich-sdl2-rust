//! Provides audio device control, configuration, wav format utilities and so on.

use static_assertions::assert_not_impl_all;
use std::{
    cell::Cell,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::MaybeUninit,
    os::raw::c_int,
};

use self::{
    format::AudioFormat,
    spec::{AudioSpec, FallbackFlag},
    status::AudioStatus,
};
use crate::{bind, Result, Sdl, SdlError};

pub use driver::*;

pub mod buffer;
mod driver;
pub mod event;
pub mod format;
pub mod queue;
pub mod spec;
pub mod status;
pub mod stream;
pub mod wav;

/// A property of an audio device.
#[derive(Debug)]
pub struct AudioDeviceProperty {
    /// The sample frequencies of an audio device..
    pub sample_freq: u32,
    /// The format of an audio device.
    pub format: AudioFormat,
    /// The numbers of channels of an audio device.
    pub channels: u8,
    /// The sample rates of an audio device.
    pub samples: u16,
}

/// A lock to prevent an audio device from calling the callback [`spec::AudioCallback`].
pub struct AudioDeviceLock<'device>(u32, PhantomData<&'device mut dyn AudioDevice>);

impl AudioDeviceLock<'_> {
    /// Unlocks the lock of an audio device.
    fn unlock(self) {
        unsafe { bind::SDL_UnlockAudioDevice(self.0) }
    }
}

/// Common methods for an audio device, such as a speaker and a microphone.
pub trait AudioDevice {
    /// Returns the id of an audio device.
    fn id(&self) -> u32;

    /// Returns the status of an audio device.
    fn status(&self) -> AudioStatus {
        unsafe { bind::SDL_GetAudioDeviceStatus(self.id()) }.into()
    }

    /// Obtains the lock preventing from calling the callback [`spec::AudioCallback`].
    fn lock(&mut self) -> AudioDeviceLock {
        unsafe { bind::SDL_LockAudioDevice(self.id()) }
        AudioDeviceLock(self.id(), PhantomData)
    }
}

/// An audio device to output sound.
pub struct SpeakerDevice {
    id: u32,
    _phantom: PhantomData<Cell<u8>>,
}

impl std::fmt::Debug for SpeakerDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpeakerDevice")
            .field("id", &self.id)
            .finish()
    }
}

assert_not_impl_all!(SpeakerDevice: Send, Sync);

impl SpeakerDevice {
    /// Returns all of speaker audio device names on now.
    pub fn all_devices() -> impl Iterator<Item = String> {
        devices(false)
    }

    /// Opens the audio device named `device` with the specification and fallback flag.
    /// If device is `None`, the default audio device is used.
    pub fn open(
        device: Option<String>,
        spec: &AudioSpec,
        fallback: FallbackFlag,
    ) -> Result<(Self, AudioDeviceProperty)> {
        let (id, prop) = open(false, device, spec, fallback)?;
        Ok((
            Self {
                id,
                _phantom: PhantomData,
            },
            prop,
        ))
    }
}

impl AudioDevice for SpeakerDevice {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for SpeakerDevice {
    fn drop(&mut self) {
        unsafe { bind::SDL_CloseAudioDevice(self.id) }
    }
}

/// An audio device to input sound.
pub struct MicrophoneDevice {
    id: u32,
    _phantom: PhantomData<Cell<u8>>,
}

impl std::fmt::Debug for MicrophoneDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MicrophoneDevice")
            .field("id", &self.id)
            .finish()
    }
}

assert_not_impl_all!(MicrophoneDevice: Send, Sync);

impl MicrophoneDevice {
    /// Returns all of microphone audio device names on now.
    pub fn all_devices() -> impl Iterator<Item = String> {
        devices(true)
    }

    /// Opens the audio device named `device` with the specification and fallback flag.
    /// If device is `None`, the default audio device is used.
    pub fn open(
        device: Option<String>,
        spec: &AudioSpec,
        fallback: FallbackFlag,
    ) -> Result<(Self, AudioDeviceProperty)> {
        let (id, prop) = open(true, device, spec, fallback)?;
        Ok((
            Self {
                id,
                _phantom: PhantomData,
            },
            prop,
        ))
    }
}

impl AudioDevice for MicrophoneDevice {
    fn id(&self) -> u32 {
        self.id
    }
}

/// Returns the device name of the id `device_id`. Please set `is_microphone` according to what type you want to.
pub fn device_name(device_id: u32, is_microphone: bool) -> Option<String> {
    let ptr = unsafe {
        bind::SDL_GetAudioDeviceName(device_id as i32, if is_microphone { 1 } else { 0 })
    };
    (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string())
}

fn devices(is_capture: bool) -> impl Iterator<Item = String> {
    let is_capture_raw = if is_capture { 1 } else { 0 };
    let num_devices = unsafe {
        bind::SDL_InitSubSystem(bind::SDL_INIT_AUDIO);
        bind::SDL_GetNumAudioDevices(is_capture_raw)
    };
    (0..num_devices).map(move |index| {
        let cstr = unsafe { CStr::from_ptr(bind::SDL_GetAudioDeviceName(index, is_capture_raw)) };
        cstr.to_string_lossy().to_string()
    })
}

fn open(
    is_capture: bool,
    device: Option<String>,
    spec: &AudioSpec,
    fallback: FallbackFlag,
) -> Result<(u32, AudioDeviceProperty)> {
    let is_capture_raw = if is_capture { 1 } else { 0 };
    let mut actual = MaybeUninit::uninit();
    let id = unsafe {
        bind::SDL_OpenAudioDevice(
            device.map_or(std::ptr::null_mut(), |s| {
                let c_string = CString::new(s).unwrap();
                c_string.into_raw()
            }),
            is_capture_raw,
            spec.raw() as *const _,
            actual.as_mut_ptr(),
            fallback.bits() as c_int,
        )
    };
    if id == 0 {
        return Err(SdlError::Others { msg: Sdl::error() });
    }
    let actual = unsafe { actual.assume_init() };
    Ok((
        id,
        AudioDeviceProperty {
            sample_freq: actual.freq as u32,
            format: actual.format.into(),
            channels: actual.channels,
            samples: actual.samples,
        },
    ))
}
