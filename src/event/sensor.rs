use std::{ffi::CStr, os::raw::c_int, ptr::NonNull};

use crate::bind;

pub struct Sensor {
    ptr: NonNull<bind::SDL_Sensor>,
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
}

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
