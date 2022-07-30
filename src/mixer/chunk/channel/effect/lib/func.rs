use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::RwLock};

use super::{Effect, PositionArgs};
use crate::{audio::format::AudioFormatFlag, mixer::device::MixSpec, Result, SdlError};

mod conv;
mod rotate;

static TABLE_U8: Lazy<RwLock<HashMap<u8, HashMap<u8, u8>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for volume in 0u8..=255 {
        let mut col = HashMap::new();
        for sample in 0u8..=255 {
            let converted = (((sample as f64 - 128.0) * (volume as f64 / 255.0)) + 128.0) as u8;
            col.insert(sample, converted);
        }
        map.insert(volume, col);
    }
    map.into()
});

static TABLE_I8: Lazy<RwLock<HashMap<u8, HashMap<u8, i8>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for volume in 0u8..=255 {
        let mut col = HashMap::new();
        for sample in 0u8..=255 {
            let converted = ((sample as f64 - 128.0) * (volume as f64 / 255.0)) as i8;
            col.insert(sample, converted);
        }
        map.insert(volume, col);
    }
    map.into()
});

macro_rules! position {
    (u8, 2ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            let table = TABLE_U8.read().unwrap();
            let mut left = table
                .get(&((256.0 * $args.gains[0]) as u8))
                .unwrap();
            let mut right = table
                .get(&((256.0 * $args.gains[1]) as u8))
                .unwrap();
            let distance = table
                .get(&((256.0 * $args.distance) as u8))
                .unwrap();
            if $args.room_angle.0 == 180 {
                std::mem::swap(&mut left, &mut right);
            }
            let left = left;
            let right = right;

            for (i, b) in buf.iter_mut().enumerate() {
                if (i % 2 == 0) ^ cfg!(target_endian = "big"){
                    *b = *distance.get(left.get(&*b).unwrap()).unwrap();
                } else {
                    *b = *distance.get(right.get(&*b).unwrap()).unwrap();
                }
            }
        })
    };
    (i8, 2ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            let table = TABLE_I8.read().unwrap();
            let mut left = table
                .get(&((256.0 * $args.gains[0]) as u8))
                .unwrap();
            let mut right = table
                .get(&((256.0 * $args.gains[1]) as u8))
                .unwrap();
            let distance = table
                .get(&((256.0 * $args.distance) as u8))
                .unwrap();
            if $args.room_angle.0 == 180 {
                std::mem::swap(&mut left, &mut right);
            }
            let left = left;
            let right = right;

            for (i, b) in buf.iter_mut().enumerate() {
                if (i % 2 == 0) ^ cfg!(target_endian = "big") {
                    *b = distance.get(&((*left.get(&*b).unwrap() as i32 + 128) as u8)).unwrap().to_ne_bytes()[0];
                } else {
                    *b = distance.get(&((*right.get(&*b).unwrap() as i32 + 128) as u8)).unwrap().to_ne_bytes()[0];
                }
            }
        })
    };
    ($target:ty, 2ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(2 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 2];
                    $crate::converter!($target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(2ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    ($target:ty, 4ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(4 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 4];
                    $crate::converter!($target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(4ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    ($target:ty, 6ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(6 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 6];
                    $crate::converter!($target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(6ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Lsb, $target:ty, 2ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(2 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 2];
                    $crate::converter!(Lsb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(2ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Lsb, $target:ty, 4ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(4 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 4];
                    $crate::converter!(Lsb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(4ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Lsb, $target:ty, 6ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(6 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 6];
                    $crate::converter!(Lsb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(6ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Msb, $target:ty, 2ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(2 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 2];
                    $crate::converter!(Msb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(2ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Msb, $target:ty, 4ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(4 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 4];
                    $crate::converter!(Msb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(4ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Msb, $target:ty, 6ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(6 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 6];
                    $crate::converter!(Msb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(6ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
}

pub(super) fn select_fn<'device>(format: MixSpec, args: PositionArgs) -> Result<Effect<'device>> {
    let channels = format.channels;
    let bit_size = format.format.bit_size;
    let is_signed = format.format.flag.contains(AudioFormatFlag::SIGNED);
    let is_msb = format.format.flag.contains(AudioFormatFlag::BIG_ENDIAN);
    if format.format.flag.contains(AudioFormatFlag::FLOAT) {
        return Ok(match channels {
            1 | 2 => position!(f32, 2ch, args),
            4 => position!(f32, 4ch, args),
            6 => position!(f32, 6ch, args),
            _ => return Err(SdlError::UnsupportedFeature),
        });
    }
    Ok(match (channels, bit_size, is_signed, is_msb) {
        (1 | 2, 8, false, _) => position!(u8, 2ch, args),
        (4, 8, false, _) => position!(u8, 4ch, args),
        (6, 8, false, _) => position!(u8, 6ch, args),
        (1 | 2, 8, true, _) => position!(i8, 2ch, args),
        (4, 8, true, _) => position!(i8, 4ch, args),
        (6, 8, true, _) => position!(i8, 6ch, args),
        (1 | 2, 16, false, false) => position!(Lsb, u16, 2ch, args),
        (4, 16, false, false) => position!(Lsb, u16, 4ch, args),
        (6, 16, false, false) => position!(Lsb, u16, 6ch, args),
        (1 | 2, 16, true, false) => position!(Lsb, i16, 2ch, args),
        (4, 16, true, false) => position!(Lsb, i16, 4ch, args),
        (6, 16, true, false) => position!(Lsb, i16, 6ch, args),
        (1 | 2, 16, false, true) => position!(Msb, u16, 2ch, args),
        (4, 16, false, true) => position!(Msb, u16, 4ch, args),
        (6, 16, false, true) => position!(Msb, u16, 6ch, args),
        (1 | 2, 16, true, true) => position!(Msb, i16, 2ch, args),
        (4, 16, true, true) => position!(Msb, i16, 4ch, args),
        (6, 16, true, true) => position!(Msb, i16, 6ch, args),
        (1 | 2, 32, _, true) => position!(Msb, i32, 2ch, args),
        (4, 32, _, true) => position!(Msb, i32, 4ch, args),
        (6, 32, _, true) => position!(Msb, i32, 6ch, args),
        (1 | 2, 32, _, false) => position!(Lsb, i32, 2ch, args),
        (4, 32, _, false) => position!(Lsb, i32, 4ch, args),
        (6, 32, _, false) => position!(Lsb, i32, 6ch, args),
        _ => return Err(SdlError::UnsupportedFeature),
    })
}

#[inline(always)]
fn apply_gains(args: &PositionArgs, gained: &mut [f64]) {
    for (out, &gain) in gained.iter_mut().zip(args.gains.iter()) {
        *out *= gain;
        *out *= args.distance;
    }
}
