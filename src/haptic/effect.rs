use crate::bind;

use super::direction::Direction;

#[derive(Debug, Clone)]
pub enum HapticEffect {
    Constant(Direction, Play, Trigger, Level, Envelope),
    Periodic(Direction, Play, Trigger, Wave, Envelope),
    Condition(Play, Trigger, Condition),
    Ramp(Direction, Play, Trigger, Ramp, Envelope),
    LeftRight {
        length: u32,
        large_magnitude: u16,
        small_magnitude: u16,
    },
    Custom(Direction, Play, Trigger, Custom, Envelope),
}

impl HapticEffect {
    pub(super) fn into_raw(self) -> bind::SDL_HapticEffect {
        match self {
            HapticEffect::Constant(dir, play, trigger, level, env) => bind::SDL_HapticEffect {
                constant: bind::SDL_HapticConstant {
                    type_: bind::SDL_HAPTIC_CONSTANT as u16,
                    direction: dir.to_raw(),
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
                    direction: dir.to_raw(),
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
                    type_: condition.kind.to_raw(),
                    direction: bind::SDL_HapticDirection {
                        type_: 0,
                        dir: [0; 3],
                    },
                    length: play.length,
                    delay: play.delay,
                    button: trigger.button,
                    interval: trigger.interval,
                    right_sat: condition.right_level.0,
                    left_sat: condition.left_level.0,
                    right_coeff: condition.right_coefficient.0,
                    left_coeff: condition.left_coefficient.0,
                    deadband: condition.dead_band.0,
                    center: condition.center.0,
                },
            },
            HapticEffect::Ramp(dir, play, trigger, ramp, env) => bind::SDL_HapticEffect {
                ramp: bind::SDL_HapticRamp {
                    type_: bind::SDL_HAPTIC_RAMP as u16,
                    direction: dir.to_raw(),
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
                    direction: dir.to_raw(),
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
                        kind: type_.into(),
                        right_level: Vector3(right_sat),
                        left_level: Vector3(left_sat),
                        right_coefficient: Vector3(right_coeff),
                        left_coefficient: Vector3(left_coeff),
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

#[derive(Debug, Clone)]
pub struct Play {
    pub length: u32,
    pub delay: u16,
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub button: u16,
    pub interval: u16,
}

#[derive(Debug, Clone)]
pub struct Level(pub i16);

#[derive(Debug, Clone)]
pub struct Envelope {
    pub attack_length: u16,
    pub attack_level: u16,
    pub fade_length: u16,
    pub fade_level: u16,
}

#[derive(Debug, Clone)]
pub struct Wave {
    pub kind: WaveKind,
    pub period: u16,
    pub magnitude: i16,
    pub offset: i16,
    pub phase: u16,
}

#[derive(Debug, Clone)]
pub enum WaveKind {
    Sine,
    Triangle,
    SawToothUp,
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

#[derive(Debug, Clone)]
pub struct Vector3<T>(pub [T; 3]);

#[derive(Debug, Clone)]
pub struct Condition {
    pub kind: ConditionKind,
    pub right_level: Vector3<u16>,
    pub left_level: Vector3<u16>,
    pub right_coefficient: Vector3<i16>,
    pub left_coefficient: Vector3<i16>,
    pub dead_band: Vector3<u16>,
    pub center: Vector3<i16>,
}

#[derive(Debug, Clone)]
pub enum ConditionKind {
    Spring,
    Damper,
    Inertia,
    Friction,
}

impl ConditionKind {
    fn to_raw(&self) -> u16 {
        (match *self {
            ConditionKind::Spring => bind::SDL_HAPTIC_SPRING,
            ConditionKind::Damper => bind::SDL_HAPTIC_DAMPER,
            ConditionKind::Inertia => bind::SDL_HAPTIC_INERTIA,
            ConditionKind::Friction => bind::SDL_HAPTIC_FRICTION,
        }) as u16
    }
}

impl From<u16> for ConditionKind {
    fn from(raw: u16) -> Self {
        match raw as u32 {
            bind::SDL_HAPTIC_SPRING => Self::Spring,
            bind::SDL_HAPTIC_DAMPER => Self::Damper,
            bind::SDL_HAPTIC_INERTIA => Self::Inertia,
            bind::SDL_HAPTIC_FRICTION => Self::Friction,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ramp {
    pub start: Level,
    pub end: Level,
}

#[derive(Debug, Clone)]
pub struct Custom {
    channels: u8,
    period: u16,
    samples: u16,
    data: Vec<u16>,
}
