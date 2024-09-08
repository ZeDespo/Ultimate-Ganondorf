//! General utility scripts that will enable Ganondorf's core function hooking.
use crate::imports::*;
use core::fmt;

#[derive(Copy, Clone)]
#[repr(C)]
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
pub const GANON_FLOAT_DURATION_WORK_INT: i32 = 0x42070;
pub const GANON_TELEPORT_INTO_FLOAT_INIT_FLAG: i32 = 0x69420;
pub const GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG: i32 = 0x69421;
pub const GANON_TELEPORT_INTO_FLOAT_WAS_CANNOT_FLOAT_FLAG: i32 = 0x69422;
pub const GANON_START_FLOAT_FLAG: i32 = 0x69423;
pub const GANON_CAN_TELEPORT_FLAG: i32 = 0x69424;
pub const GANON_PRE_FLOAT_MUTEX: i32 = 0x69425;
pub const GANON_DOWN_TILT_2_FLAG: i32 = 0x69426;
pub const FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT: i32 = 0x1ED;
pub const FIGHTER_GANON_STATUS_KIND_BACKHAND: i32 = 0x1EE;

pub trait BomaExt {
    unsafe fn prev_status_kind(&mut self) -> i32;
    unsafe fn status_kind(&mut self) -> i32;

    // You could do something like this
    unsafe fn is_status(&mut self, kind: i32) -> bool;

    unsafe fn situation_kind(&mut self) -> i32;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;

    unsafe fn motion_kind(&mut self) -> u64;
    unsafe fn entry_id(&mut self) -> usize;
    unsafe fn motion_module_frame(&mut self) -> f32;
    unsafe fn kinetic_kind(&mut self) -> i32;

    unsafe fn teleport_into_float(&mut self) -> bool;
    unsafe fn start_float(&mut self) -> bool;

    unsafe fn jump_button_pressed(&mut self) -> bool;
    // Or could do
    // unsafe fn is_button_on(&mut self, button type) -> bool;
    // And pass in the jump button

    unsafe fn get_float_duration(&mut self) -> i16;
    unsafe fn set_float_duration(&mut self, n: i32);
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn prev_status_kind(&mut self) -> i32 {
        StatusModule::prev_status_kind(self, 0)
    }

    unsafe fn status_kind(&mut self) -> i32 {
        StatusModule::status_kind(self)
    }

    unsafe fn is_status(&mut self, kind: i32) -> bool {
        self.status_kind() == kind
    }

    unsafe fn situation_kind(&mut self) -> i32 {
        StatusModule::situation_kind(self)
    }

    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        self.situation_kind() == kind
    }

    unsafe fn motion_kind(&mut self) -> u64 {
        MotionModule::motion_kind(self)
    }

    unsafe fn entry_id(&mut self) -> usize {
        WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
    }

    unsafe fn motion_module_frame(&mut self) -> f32 {
        MotionModule::frame(self)
    }

    unsafe fn kinetic_kind(&mut self) -> i32 {
        KineticModule::get_kinetic_type(self)
    }

    unsafe fn teleport_into_float(&mut self) -> bool {
        in_teleport(self)
    }

    unsafe fn start_float(&mut self) -> bool {
        WorkModule::is_flag(self, GANON_START_FLOAT_FLAG)
    }

    unsafe fn jump_button_pressed(&mut self) -> bool {
        ControlModule::check_button_on(self, *CONTROL_PAD_BUTTON_JUMP)
    }
    unsafe fn get_float_duration(&mut self) -> i16 {
        WorkModule::get_int(self, GANON_FLOAT_DURATION_WORK_INT) as i16
    }
    unsafe fn set_float_duration(&mut self, n: i32) {
        WorkModule::set_int(self, n, GANON_FLOAT_DURATION_WORK_INT);
    }
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
