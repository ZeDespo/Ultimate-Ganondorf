//! General utility scripts that will enable Ganondorf's core function hooking.
use core::fmt;
use crate::imports::*;

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

#[derive(Copy, Clone)]
pub enum FloatActivationStatus {
    Waiting,
    Jump(i8),
    JumpUsed,
    JumpAerial(i8),
    JumpAerialUsed,
    NotApplicable,
}

impl fmt::Display for FloatActivationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloatActivationStatus::NotApplicable => write!(f, "'Not Applicable'"),
            FloatActivationStatus::Waiting => write!(f, "'Waiting'"),
            FloatActivationStatus::Jump(i) => write!(f, "'Jump({})'", i),
            FloatActivationStatus::JumpUsed => write!(f, "'JumpUsed'"),
            FloatActivationStatus::JumpAerial(i) => write!(f, "'JumpAerial({})'", i),
            FloatActivationStatus::JumpAerialUsed => write!(f, "'JumpAerialUsed'"),
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
    pub kinetic_kind: i32,
    pub teleport_into_float: bool,
    pub start_float: bool,
    pub jump_button_pressed: bool,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

impl Position2D {
    pub fn to_vector3f(self: Self) -> Vector3f {
        Vector3f {
            x: self.x,
            y: self.y,
            z: 0.0,
        }
    }

    pub fn neutral() -> Position2D {
        Position2D { x: 0.0, y: 0.0 }
    }
}

#[derive(Copy, Clone)]
pub struct GanonState {
    pub float_status: FloatStatus,
    pub float_speed: Position2D,
    pub float_activation_status: FloatActivationStatus,
    pub teleport_direction: Position2D,
}

#[repr(i32)]
#[derive(Debug)]
pub enum TeleportStatus {
    Ready = 0,
    PreTransit,
    Transit,
    EndTransit,
    End,
}

impl fmt::Display for TeleportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TeleportStatus {
    /// Note: Not using `transmute` becuse we can get values outside of bounds.
    pub fn from_int(value: i32) -> TeleportStatus {
        match value {
            1 => TeleportStatus::PreTransit,
            2 => TeleportStatus::Transit,
            3 => TeleportStatus::EndTransit,
            4 => TeleportStatus::End,
            _ => TeleportStatus::Ready,
        }
    }
}

pub static mut GS: [GanonState; 8] = [GanonState {
    float_status: FloatStatus::CanFloat,
    float_speed: Position2D { x: 0.0, y: 0.0 },
    teleport_direction: Position2D { x: 0.0, y: 0.0 },
    float_activation_status: FloatActivationStatus::Waiting,
}; 8];

pub const GANON_TELEPORT_WORK_INT: i32 = 0x42069;
pub const GANON_TELEPORT_INTO_FLOAT_INIT_FLAG: i32 = 0x69420;
pub const GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG: i32 = 0x69421;
pub const GANON_TELEPORT_INTO_FLOAT_WAS_CANNOT_FLOAT_FLAG: i32 = 0x69422;
pub const GANON_DOWN_SPECIAL_AIR: i32 = 0x69423;
pub const GANON_DARK_RUPTURE_ACTIVE: i32 = 0x69424;
pub const GANON_START_FLOAT_FLAG: i32 = 0x69425;
pub const GANON_CAN_TELEPORT_FLAG: i32 = 0x69426;
pub const GANON_DOWN_SPECIAL_GROUND: i32 = 0x69427;
pub const GANON_PRE_FLOAT_MUTEX: i32 = 0x69428;
pub const FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT: i32 = 0x1ED;
pub const FIGHTER_GANON_STATUS_KIND_BACKHAND: i32 = 0x1EE;

pub unsafe extern "C" fn in_dive(boma: *mut BattleObjectModuleAccessor) -> bool {
    WorkModule::is_flag(boma, GANON_DOWN_SPECIAL_AIR)
}

/// Convenience function for checking teleport status via a handler flag.
pub unsafe extern "C" fn in_teleport(boma: *mut BattleObjectModuleAccessor) -> bool {
    WorkModule::is_flag(boma, GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG)
}

pub unsafe extern "C" fn triforce_hand_fx(agent: &mut L2CAgentBase, rate: f32) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("ganon_final_hand_triforce"),
            Hash40::new("handr"),
            2.5,
            1,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, rate);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}
