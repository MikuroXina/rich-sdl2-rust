//! Setup and reading data from other sensors.

use std::{ffi::CStr, os::raw::c_int, ptr::NonNull};

use crate::bind;

/// A kind of the other sensors.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SensorKind {
    /// The others unrecognized by SDL2.
    Others(i32),
    /// The accelerometer.
    Accel,
    /// The gyroscope.
    Gyro,
}

/// A sensor loaded by SDL2.
pub struct Sensor {
    ptr: NonNull<bind::SDL_Sensor>,
}

impl std::fmt::Debug for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sensor")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl Sensor {
    /// Reads some `f32` data by count
    #[must_use]
    pub fn data(&self, read_count: usize) -> Vec<f32> {
        let mut data = vec![0.0; read_count];
        unsafe {
            bind::SDL_SensorGetData(self.ptr.as_ptr(), data.as_mut_ptr(), read_count as c_int);
        }
        data
    }

    /// Returns the name of the sensor
    #[must_use]
    pub fn name(&self) -> &str {
        let cstr = unsafe { CStr::from_ptr(bind::SDL_SensorGetName(self.ptr.as_ptr())) };
        cstr.to_str().unwrap()
    }

    /// Returns the kind of the sensor
    #[must_use]
    pub fn kind(&self) -> SensorKind {
        let ty = unsafe { bind::SDL_SensorGetType(self.ptr.as_ptr()) };
        match ty {
            bind::SDL_SENSOR_ACCEL => SensorKind::Accel,
            bind::SDL_SENSOR_GYRO => SensorKind::Gyro,
            bind::SDL_SENSOR_UNKNOWN => {
                let ty = unsafe { bind::SDL_SensorGetNonPortableType(self.ptr.as_ptr()) };
                SensorKind::Others(ty)
            }
            _ => unreachable!(),
        }
    }
}

/// A set of `Sensor`, containing other sensors.
#[derive(Debug)]
pub struct SensorSet(Vec<Sensor>);

impl SensorSet {
    /// Setup the system and recognizes the sensors.
    #[must_use]
    pub fn new() -> Self {
        let sensor_count = unsafe {
            bind::SDL_InitSubSystem(bind::SDL_INIT_SENSOR);
            bind::SDL_NumSensors()
        };
        Self(
            (0..sensor_count)
                .map(|index| {
                    let sensor = unsafe { bind::SDL_SensorOpen(index) };
                    Sensor {
                        ptr: NonNull::new(sensor).expect("opening sensor failed"),
                    }
                })
                .collect(),
        )
    }

    /// Returns the sensors slice.
    #[must_use]
    pub fn sensors(&self) -> &[Sensor] {
        &self.0
    }
}

impl Default for SensorSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SensorSet {
    fn drop(&mut self) {
        for sensor in &self.0 {
            unsafe { bind::SDL_SensorClose(sensor.ptr.as_ptr()) }
        }
    }
}
