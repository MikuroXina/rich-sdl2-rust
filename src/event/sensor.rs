use std::{ffi::CStr, os::raw::c_int, ptr::NonNull};

use crate::bind;

#[derive(Debug, Clone)]
pub enum SensorKind {
    Others(i32),
    Accel,
    Gyro,
}

pub struct Sensor {
    ptr: NonNull<bind::SDL_Sensor>,
}

impl std::fmt::Debug for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sensor")
            .field("name", &self.name())
            .finish()
    }
}

impl Sensor {
    pub fn data(&self, read_count: usize) -> Vec<f32> {
        let mut data = vec![0.0; read_count];
        unsafe {
            bind::SDL_SensorGetData(self.ptr.as_ptr(), data.as_mut_ptr(), read_count as c_int);
        }
        data
    }

    pub fn name(&self) -> &str {
        let cstr = unsafe { CStr::from_ptr(bind::SDL_SensorGetName(self.ptr.as_ptr())) };
        cstr.to_str().unwrap()
    }

    pub fn kind(&self) -> SensorKind {
        let ty = unsafe { bind::SDL_SensorGetType(self.ptr.as_ptr()) };
        match ty {
            bind::SDL_SensorType_SDL_SENSOR_ACCEL => SensorKind::Accel,
            bind::SDL_SensorType_SDL_SENSOR_GYRO => SensorKind::Gyro,
            bind::SDL_SensorType_SDL_SENSOR_UNKNOWN => {
                let ty = unsafe { bind::SDL_SensorGetNonPortableType(self.ptr.as_ptr()) };
                SensorKind::Others(ty)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct SensorSet(Vec<Sensor>);

impl SensorSet {
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

    pub fn sensors(&self) -> &[Sensor] {
        &self.0
    }
}

impl Drop for SensorSet {
    fn drop(&mut self) {
        for sensor in &self.0 {
            unsafe { bind::SDL_SensorClose(sensor.ptr.as_ptr()) }
        }
    }
}
