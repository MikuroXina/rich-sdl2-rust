//! Controls the effect on the haptic device.

use crate::bind;

use super::direction::Direction;

/// An effect on the haptic device.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HapticEffect {
    /// Applies the constant force in the direction.
    Constant(Direction, Play, Trigger, Level, Envelope),
    /// Applies the force of the periodic waveform.
    Periodic(Direction, Play, Trigger, Wave, Envelope),
    /// Applies the force on fulfilled the condition.
    Condition(Play, Trigger, Condition),
    /// Applies the force by linear ramp.
    Ramp(Direction, Play, Trigger, Ramp, Envelope),
    /// Applies the left/right effect with two motors magnitude. One motor is high frequency, the other is low frequency.
    LeftRight {
        /// The length of the playing in milliseconds.
        length: u32,
        /// The magnitude for the large motor.
        large_magnitude: u16,
        /// The magnitude for the small motor.
        small_magnitude: u16,
    },
    /// Applies the force of the periodic waveform from your sampled data.
    Custom(Direction, Play, Trigger, Custom, Envelope),
}

impl HapticEffect {
    pub(super) fn into_raw(self) -> bind::SDL_HapticEffect {
        match self {
            HapticEffect::Constant(dir, play, trigger, level, env) => bind::SDL_HapticEffect {
                constant: bind::SDL_HapticConstant {
                    type_: bind::SDL_HAPTIC_CONSTANT as u16,
                    direction: dir.into_raw(),
                    length: play.length,
                    delay: play.delay,
                    button: trigger.button,
                    interval: trigger.interval,
                    level: level.0,
                    attack_length: env.attack_length,
                    attack_level: env.attack_level,
                    fade_length: env.fade_length,
                    fade_level: env.fade_level,
                },
            },
            HapticEffect::Periodic(dir, play, trigger, wave, env) => bind::SDL_HapticEffect {
                periodic: bind::SDL_HapticPeriodic {
                    type_: wave.kind.to_raw(),
                    direction: dir.into_raw(),
                    length: play.length,
                    delay: play.delay,
                    button: trigger.button,
                    interval: trigger.interval,
                    period: wave.period,
                    magnitude: wave.magnitude,
                    offset: wave.offset,
                    phase: wave.phase,
                    attack_length: env.attack_length,
                    attack_level: env.attack_level,
                    fade_length: env.fade_length,
                    fade_level: env.fade_level,
                },
            },
            HapticEffect::Condition(play, trigger, condition) => bind::SDL_HapticEffect {
                condition: bind::SDL_HapticCondition {
                    type_: 0,
                    direction: bind::SDL_HapticDirection {
                        type_: 0,
                        dir: [0; 3],
                    },
                    length: play.length,
                    delay: play.delay,
                    button: trigger.button,
                    interval: trigger.interval,
                    right_sat: condition.positive_level.0,
                    left_sat: condition.negative_level.0,
                    right_coeff: condition.positive_coefficient.0,
                    left_coeff: condition.negative_coefficient.0,
                    deadband: condition.dead_band.0,
                    center: condition.center.0,
                },
            },
            HapticEffect::Ramp(dir, play, trigger, ramp, env) => bind::SDL_HapticEffect {
                ramp: bind::SDL_HapticRamp {
                    type_: bind::SDL_HAPTIC_RAMP as u16,
                    direction: dir.into_raw(),
                    length: play.length,
                    delay: play.delay,
                    button: trigger.button,
                    interval: trigger.interval,
                    start: ramp.start.0,
                    end: ramp.end.0,
                    attack_length: env.attack_length,
                    attack_level: env.attack_level,
                    fade_length: env.fade_length,
                    fade_level: env.fade_level,
                },
            },
            HapticEffect::LeftRight {
                length,
                large_magnitude,
                small_magnitude,
            } => bind::SDL_HapticEffect {
                leftright: bind::SDL_HapticLeftRight {
                    type_: bind::SDL_HAPTIC_LEFTRIGHT as u16,
                    length,
                    large_magnitude,
                    small_magnitude,
                },
            },
            HapticEffect::Custom(dir, play, trigger, mut custom, env) => bind::SDL_HapticEffect {
                custom: bind::SDL_HapticCustom {
                    type_: bind::SDL_HAPTIC_CUSTOM as u16,
                    direction: dir.into_raw(),
                    length: play.length,
                    delay: play.delay,
                    button: trigger.button,
                    interval: trigger.interval,
                    channels: custom.channels,
                    period: custom.period,
                    samples: custom.samples,
                    data: custom.data.as_mut_ptr(),
                    attack_length: env.attack_length,
                    attack_level: env.attack_level,
                    fade_length: env.fade_length,
                    fade_level: env.fade_level,
                },
            },
        }
    }
}

impl From<bind::SDL_HapticEffect> for HapticEffect {
    fn from(raw: bind::SDL_HapticEffect) -> Self {
        match unsafe { raw.type_ } as u32 {
            bind::SDL_HAPTIC_CONSTANT => {
                let bind::SDL_HapticConstant {
                    direction,
                    length,
                    delay,
                    button,
                    interval,
                    level,
                    attack_length,
                    attack_level,
                    fade_length,
                    fade_level,
                    ..
                } = unsafe { raw.constant };
                Self::Constant(
                    direction.into(),
                    Play { length, delay },
                    Trigger { button, interval },
                    Level(level),
                    Envelope {
                        attack_length,
                        attack_level,
                        fade_length,
                        fade_level,
                    },
                )
            }
            bind::SDL_HAPTIC_SINE
            | bind::SDL_HAPTIC_TRIANGLE
            | bind::SDL_HAPTIC_SAWTOOTHUP
            | bind::SDL_HAPTIC_SAWTOOTHDOWN => {
                let bind::SDL_HapticPeriodic {
                    type_,
                    direction,
                    length,
                    delay,
                    button,
                    interval,
                    period,
                    magnitude,
                    offset,
                    phase,
                    attack_length,
                    attack_level,
                    fade_length,
                    fade_level,
                    ..
                } = unsafe { raw.periodic };
                Self::Periodic(
                    direction.into(),
                    Play { length, delay },
                    Trigger { button, interval },
                    Wave {
                        kind: type_.into(),
                        period,
                        magnitude,
                        offset,
                        phase,
                    },
                    Envelope {
                        attack_length,
                        attack_level,
                        fade_length,
                        fade_level,
                    },
                )
            }
            bind::SDL_HAPTIC_SPRING
            | bind::SDL_HAPTIC_DAMPER
            | bind::SDL_HAPTIC_INERTIA
            | bind::SDL_HAPTIC_FRICTION => {
                let bind::SDL_HapticCondition {
                    type_,
                    length,
                    delay,
                    button,
                    interval,
                    right_sat,
                    left_sat,
                    right_coeff,
                    left_coeff,
                    deadband,
                    center,
                    ..
                } = unsafe { raw.condition };
                Self::Condition(
                    Play { length, delay },
                    Trigger { button, interval },
                    Condition {
                        positive_level: Vector3(right_sat),
                        negative_level: Vector3(left_sat),
                        positive_coefficient: Vector3(right_coeff),
                        negative_coefficient: Vector3(left_coeff),
                        dead_band: Vector3(deadband),
                        center: Vector3(center),
                    },
                )
            }
            bind::SDL_HAPTIC_RAMP => {
                let bind::SDL_HapticRamp {
                    direction,
                    length,
                    delay,
                    button,
                    interval,
                    start,
                    end,
                    attack_length,
                    attack_level,
                    fade_length,
                    fade_level,
                    ..
                } = unsafe { raw.ramp };
                Self::Ramp(
                    direction.into(),
                    Play { length, delay },
                    Trigger { button, interval },
                    Ramp {
                        start: Level(start),
                        end: Level(end),
                    },
                    Envelope {
                        attack_length,
                        attack_level,
                        fade_length,
                        fade_level,
                    },
                )
            }
            bind::SDL_HAPTIC_LEFTRIGHT => {
                let bind::SDL_HapticLeftRight {
                    length,
                    large_magnitude,
                    small_magnitude,
                    ..
                } = unsafe { raw.leftright };
                Self::LeftRight {
                    length,
                    large_magnitude,
                    small_magnitude,
                }
            }
            bind::SDL_HAPTIC_CUSTOM => {
                let bind::SDL_HapticCustom {
                    direction,
                    length,
                    delay,
                    button,
                    interval,
                    channels,
                    period,
                    samples,
                    data,
                    attack_length,
                    attack_level,
                    fade_length,
                    fade_level,
                    ..
                } = unsafe { raw.custom };
                Self::Custom(
                    direction.into(),
                    Play { length, delay },
                    Trigger { button, interval },
                    Custom {
                        channels,
                        period,
                        samples,
                        data: unsafe {
                            std::slice::from_raw_parts_mut(
                                data,
                                channels as usize * samples as usize,
                            )
                        }
                        .to_vec(),
                    },
                    Envelope {
                        attack_length,
                        attack_level,
                        fade_length,
                        fade_level,
                    },
                )
            }
            _ => unreachable!(),
        }
    }
}

/// Length and delay of the playing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Play {
    /// The length of the playing in milliseconds.
    pub length: u32,
    /// The delay of the playing.
    pub delay: u16,
}

/// A trigger button to start the effect, and an interval between the effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trigger {
    /// The trigger button to start the effect
    pub button: u16,
    /// The interval between the effect in milliseconds.
    pub interval: u16,
}

/// A magnitude level of the force.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level(pub i16);

/// An envelope to fade in/out the effect. If both `attack_length` and `fade_level` are `0`, the envelope is not used. An example of a constant effect evolution in time:
///
/// ```text
/// Strength
/// ^
/// |
/// |      effect level --> _________________
/// |                      /                 \
/// |                     /                   \
/// |                    /                     \
/// |                   /                       \
/// | attack_level --> |                         \
/// |                  |                          | <--- fade_level
/// |
/// +--------------------------------------------------> Time
///
///                    [--]                 [---]
///                attack_length          fade_length
///
/// [------------------][-----------------------]
///        delay                 length
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    /// The length of the level from `attack_level` to the effect level in milliseconds.
    pub attack_length: u16,
    /// The initial force level on applying.
    pub attack_level: u16,
    /// The length of the level from the effect level to `attack_level`in milliseconds.
    pub fade_length: u16,
    /// The end force level on applying.
    pub fade_level: u16,
}

/// A periodic waveform specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wave {
    /// The kind of the waveform.
    pub kind: WaveKind,
    /// The period of the waveform.
    pub period: u16,
    /// The magnitude by peak-to-peak level of the waveform.
    pub magnitude: i16,
    /// The amplifier offset, mean value of the waveform.
    pub offset: i16,
    /// The phase shift in degrees times 100.
    pub phase: u16,
}

/// A kind of the waveform.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WaveKind {
    /// A sine wave like:
    ///
    /// ```text
    ///   __      __      __      __
    ///  /  \    /  \    /  \    /
    /// /    \__/    \__/    \__/
    /// ```
    Sine,
    /// A triangle wave like:
    ///
    /// ```text
    ///   /\    /\    /\    /\    /\
    ///  /  \  /  \  /  \  /  \  /
    /// /    \/    \/    \/    \/
    /// ```
    Triangle,
    /// An upwards sawtooth wave like:
    ///
    /// ```text
    ///   /|  /|  /|  /|  /|  /|  /|
    ///  / | / | / | / | / | / | / |
    /// /  |/  |/  |/  |/  |/  |/  |
    /// ```
    SawToothUp,
    /// A downwards sawtooth wave like:
    ///
    /// ```text
    /// \  |\  |\  |\  |\  |\  |\  |
    ///  \ | \ | \ | \ | \ | \ | \ |
    ///   \|  \|  \|  \|  \|  \|  \|
    /// ```
    SwaToothDown,
}

impl WaveKind {
    fn to_raw(&self) -> u16 {
        (match *self {
            WaveKind::Sine => bind::SDL_HAPTIC_SINE,
            WaveKind::Triangle => bind::SDL_HAPTIC_TRIANGLE,
            WaveKind::SawToothUp => bind::SDL_HAPTIC_SAWTOOTHUP,
            WaveKind::SwaToothDown => bind::SDL_HAPTIC_SAWTOOTHDOWN,
        }) as u16
    }
}

impl From<u16> for WaveKind {
    fn from(raw: u16) -> Self {
        match raw as u32 {
            bind::SDL_HAPTIC_SINE => Self::Sine,
            bind::SDL_HAPTIC_TRIANGLE => Self::Triangle,
            bind::SDL_HAPTIC_SAWTOOTHUP => Self::SawToothUp,
            bind::SDL_HAPTIC_SAWTOOTHDOWN => Self::SwaToothDown,
            _ => unreachable!(),
        }
    }
}

/// A vector to represent the XYZ component for [`Condition`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector3<T>(pub [T; 3]);

/// A condition to trigger the effect. Refer to [`Direction`] for which side is the positive/negative on the joystick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    /// The level when the joystick in the positive side.
    pub positive_level: Vector3<u16>,
    /// The level when the joystick in the negative side.
    pub negative_level: Vector3<u16>,
    /// How fast to increase the force towards the positive side.
    pub positive_coefficient: Vector3<i16>,
    /// How fast to increase the force towards the negative side.
    pub negative_coefficient: Vector3<i16>,
    /// The size of the dead zone.
    pub dead_band: Vector3<u16>,
    /// The position of the dead zone.
    pub center: Vector3<i16>,
}

/// A linear ramp to interpolate the force of the effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ramp {
    /// The level at the start to play.
    pub start: Level,
    /// The level at the end to play.
    pub end: Level,
}

/// A custom periodic waveform by your sampled data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Custom {
    /// The numbers of using axes.
    pub channels: u8,
    /// The period of your sampled data.
    pub period: u16,
    /// The numbers of the samples in your data.
    pub samples: u16,
    /// The sampled data. Its length must be equal to `channels * samples`.
    pub data: Vec<u16>,
}
