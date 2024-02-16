//! General utility functions necessary for the mod to work.
use core::fmt;

#[derive(Copy, Clone)]
pub enum FloatStatus {
    Floating(i16),
    CanFloat,
    CannotFloat,
}

impl fmt::Display for FloatStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloatStatus::CanFloat => write!(f, "'CanFloat'"),
            FloatStatus::CannotFloat => write!(f, "'CannotFloat'"),
            FloatStatus::Floating(i) => write!(f, "'Floating({})'", i),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

impl Speed {
    pub fn reset() -> Speed {
        Speed { x: 0.0, y: 0.0 }
    }
}

#[derive(Copy, Clone)]
pub struct GanonState {
    pub fs: FloatStatus,
    pub speed: Speed,
}

pub static mut GS: [GanonState; 8] = [GanonState {
    fs: FloatStatus::CanFloat,
    speed: Speed { x: 0.0, y: 0.0 },
}; 8];
