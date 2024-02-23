//! General utility scripts that will enable Ganondorf's core function hooking.
use core::fmt;

use smash::app::lua_bind::WorkModule;
use smash::app::BattleObjectModuleAccessor;

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

/// A convenience struct that holds necessary values. It beats having a function
/// accept numerous parameters.
#[derive(Debug)]
pub struct InitValues {
    pub prev_status_kind: i32,
    pub status_kind: i32,
    pub situation_kind: i32,
    pub motion_kind: u64,
    pub entry_id: usize,
    pub motion_module_frame: f32,
    pub teleport_into_float: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

impl Position2D {
    pub fn reset() -> Position2D {
        Position2D { x: 0.0, y: 0.0 }
    }
}

#[derive(Copy, Clone)]
pub struct GanonState {
    pub fs: FloatStatus,
    pub speed: Position2D,
}

#[repr(i32)]
#[derive(Debug)]
pub enum TeleportStatus {
    NotApplicable = 0,
    Start,
    PreTransit,
    Transit,
    End,
}

impl fmt::Display for TeleportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TeleportStatus {
    pub fn from_int(value: i32) -> TeleportStatus {
        let tp: TeleportStatus = unsafe { ::std::mem::transmute(value) };
        return tp;
    }

    pub fn to_int(self: Self) -> i32 {
        self as i32
    }
}

pub static mut GS: [GanonState; 8] = [GanonState {
    fs: FloatStatus::CanFloat,
    speed: Position2D { x: 0.0, y: 0.0 },
}; 8];

pub const GANON_TELEPORT_WORK_INT: i32 = 0x42069;
pub const GANON_TELEPORT_INTO_FLOAT_INIT_FLAG: i32 = 0x69420;
pub const GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG: i32 = 0x69421;
pub const GANON_TELEPORT_NEW_X_POS: i32 = 0x42068;
pub const GANON_TELEPORT_NEW_Y_POS: i32 = 0x42067;

/// Convenience function for checking teleport status via a handler flag.
pub unsafe extern "C" fn in_teleport(boma: *mut BattleObjectModuleAccessor) -> bool {
    WorkModule::is_flag(boma, GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG)
}
