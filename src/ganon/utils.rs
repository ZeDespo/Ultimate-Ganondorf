use core::fmt;

use skyline_smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash_script::macros;
use {
    smash::{app::lua_bind::*, hash40, lua2cpp::*},
    smashline::*,
};

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
    pub has_attacked: bool,
}

pub static mut GS: [GanonState; 8] = [GanonState {
    fs: FloatStatus::CanFloat,
    speed: Speed { x: 0.0, y: 0.0 },
    has_attacked: false,
}; 8];
