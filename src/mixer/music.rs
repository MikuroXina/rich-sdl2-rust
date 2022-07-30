//! The music control that are suitable to background music.

use std::{ffi::CString, marker::PhantomData, ptr::NonNull};

use self::{pause::Pauser, ty::MusicType};
use crate::{bind, mixer::device::MixDevice, Result, Sdl, SdlError};

pub mod custom;
pub mod pause;
pub mod ty;

/// A fading state of the music.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadingState {
    /// A music is not fading.
    None,
    /// A music is fading out.
    FadingOut,
    /// A music is fading in.
    FadingIn,
}

/// A music buffer of the audio data.
pub struct MixMusic<'device> {
    ptr: NonNull<bind::Mix_Music>,
    _phantom: PhantomData<&'device MixDevice<'device>>,
}

impl<'device> MixMusic<'device> {
    /// Constructs a music from the file, or `Err` on failure.
    ///
    /// # Panics
    ///
    /// Panics if `file_name` is empty.
    pub fn new(_device: &'device MixDevice<'device>, file_name: &str) -> Result<Self> {
        let cstr = CString::new(file_name).expect("file_name must not be empty");
        let ptr = unsafe { bind::Mix_LoadMUS(cstr.as_ptr()) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the type of the music.
    pub fn music_type(&self) -> MusicType {
        let raw = unsafe { bind::Mix_GetMusicType(self.ptr.as_ptr()) };
        MusicType::from_raw(raw)
    }

    /// Constructs a music from the file with the custom player command, or `Err` on failure.
    ///
    /// The command must handle the
    /// signals emitted by the SDL2_mixer:
    /// - On stop: `SIGTERM` signal
    /// - On pause: `SIGSTOP` signal
    /// - On unpause: `SIGCONT` signal
    ///
    /// # Panics
    ///
    /// Panics if `file_name` or `command` is empty.
    pub fn with_cmd(
        device: &'device MixDevice<'device>,
        file_name: &str,
        command: &str,
    ) -> Result<Self> {
        let cmd_cstr = CString::new(command).expect("cmd must not be empty");
        let ret = unsafe { bind::Mix_SetMusicCMD(cmd_cstr.as_ptr()) };
        if ret == -1 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Self::new(device, file_name)
    }

    /// Plays the music. If a music is already playing, it synchronously waits until the music ends.
    /// If `loops` is `None`, the play continues infinitely.
    pub fn play(&self, loops: Option<u32>) -> Result<()> {
        let ret = unsafe { bind::Mix_PlayMusic(self.ptr.as_ptr(), loops.map_or(-1, |n| n as _)) };
        if ret == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Plays the music with fade-in times in milliseconds and begin times in seconds. If a music is already playing, it synchronously waits until the music ends.
    /// If `loops` is `None`, the play continues infinitely.
    /// If `begin` is `None`, the play begins from the start.
    pub fn fade_in(&self, fade_in: u32, loops: Option<u32>, begin: Option<f64>) -> Result<()> {
        let begin = self.music_type().convert_pos(begin.unwrap_or(0.0));
        let ret = unsafe {
            bind::Mix_FadeInMusicPos(
                self.ptr.as_ptr(),
                loops.map_or(-1, |n| n as _),
                fade_in as _,
                begin,
            )
        };
        if ret == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Sets the music position in seconds, or `Err` on failure.
    pub fn set_pos(&self, pos: f64) -> Result<()> {
        let pos = self.music_type().convert_pos(pos);
        let ret = unsafe { bind::Mix_SetMusicPosition(pos) };
        if ret == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Returns the volume of the music.
    pub fn volume(&self) -> u32 {
        unsafe { bind::Mix_VolumeMusic(-1) as _ }
    }

    /// Sets the volume of the music. The `volume` is clamped in `0..=128`.
    pub fn set_volume(&self, volume: u32) {
        let _ = unsafe { bind::Mix_VolumeMusic(volume.clamp(0, 128) as _) };
    }

    /// Rewinds the music to the beginning. Rewinding is valid only mod, ogg vorbis, mpeg-1 layer-3, and midi format.
    pub fn rewind(&self) {
        unsafe { bind::Mix_RewindMusic() }
    }

    /// Pauses the music until dropping the [`Pauser`].
    pub fn pause(&'device mut self) -> Pauser<'device> {
        Pauser::pause(self)
    }

    /// Halts the music.
    pub fn halt(&self) {
        let _ = unsafe { bind::Mix_HaltMusic() };
    }

    /// Halts the music with fade-out in milliseconds.
    pub fn fade_out(&self, fade_out: u32) -> Result<()> {
        let ret = unsafe { bind::Mix_FadeOutMusic(fade_out as _) };
        if ret == 0 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }

    /// Returns whether the music is playing.
    pub fn is_playing(&self) -> bool {
        unsafe { bind::Mix_PlayingMusic() == 1 }
    }

    /// Returns the fading state of the music.
    pub fn fading_state(&self) -> FadingState {
        match unsafe { bind::Mix_FadingMusic() } {
            bind::MIX_NO_FADING => FadingState::None,
            bind::MIX_FADING_OUT => FadingState::FadingOut,
            bind::MIX_FADING_IN => FadingState::FadingIn,
            _ => unreachable!(),
        }
    }
}

impl Drop for MixMusic<'_> {
    fn drop(&mut self) {
        unsafe {
            bind::Mix_SetMusicCMD(std::ptr::null());
            bind::Mix_FreeMusic(self.ptr.as_ptr());
        }
    }
}
