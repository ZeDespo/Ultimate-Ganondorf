//! Miscellaneous variables, utility functions, and others to help
//! facilitate critical functions to the mod.
use core::fmt;

use skyline_smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use {
    smash::{app::lua_bind::*, hash40, lua2cpp::*, lua_const::*},
    smashline::*,
};

const MAX_FLOAT_FRAMES: i16 = 90;
const STARTING_FLOAT_FRAME: f32 = 2.0;

unsafe extern "C" fn check_jump(boma: *mut BattleObjectModuleAccessor) -> bool {
    if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)
        || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)
    {
        return true;
    }
    return false;
}

unsafe extern "C" fn transition_to_fall(boma: *mut BattleObjectModuleAccessor, status_kind: &i32) {
    if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR
        && [
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
        ]
        .contains(status_kind)
            == false
    {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
}

// 0 == CAN FLOAT
// 1 == CANNOT FLOAT
// > 1 == CURRENTLY FLOATING
#[derive(Copy, Clone)]
enum FloatStatus {
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

impl FloatStatus {
    fn decrement(self: Self) -> Self {
        match self {
            FloatStatus::Floating(i) => {
                let j = i - 1;
                if j == 1 {
                    return FloatStatus::CannotFloat;
                } else {
                    return FloatStatus::Floating(j);
                }
            }
            _ => return self,
        }
    }

    // Floating status should be set when:
    // 1) Ganon is on the ground
    // 2) Ganon does a special move in the air
    // 3) Ganon loses a stock
    // 4) The match is over.
    fn transition_to_can_float_if_able(
        self: Self,
        status_kind: &i32,
        situation_kind: i32,
        is_ready_go: bool,
    ) -> FloatStatus {
        if let FloatStatus::CannotFloat = self {
            if [
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH,
                *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
                *FIGHTER_STATUS_KIND_WIN,
                *FIGHTER_STATUS_KIND_LOSE,
                *FIGHTER_STATUS_KIND_DEAD,
            ]
            .contains(status_kind)
                || situation_kind != SITUATION_KIND_AIR
                || is_ready_go == false
            {
                return FloatStatus::CanFloat;
            }
        }
        return self;
    }

    // Floating should be disabled when:
    // 1) Ganon floated for the maximum time allotted
    // 2) Ganon cancels the floating with another jump.
    // 3) Ganon performs an air dodge at any time.
    fn transition_to_cannot_float_if_able(
        self: Self,
        status_kind: &i32,
        check_special_button_off: bool,
        is_jump: bool,
    ) -> FloatStatus {
        match self {
            FloatStatus::Floating(i) => {
                if i == 1 || check_special_button_off || is_jump {
                    return FloatStatus::CannotFloat;
                }
            }
            _ => {
                if [
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
                    *FIGHTER_STATUS_KIND_JUMP_AERIAL,
                ]
                .contains(status_kind)
                {
                    return FloatStatus::CannotFloat;
                }
            }
        }
        return self;
    }

    fn transition_to_floating_if_able(self: Self, motion_module_frame: &f32) -> FloatStatus {
        if let FloatStatus::CanFloat = self {
            if *motion_module_frame == STARTING_FLOAT_FRAME {
                return FloatStatus::Floating(MAX_FLOAT_FRAMES);
            }
        }
        return self;
    }
}

static mut FLOAT: [FloatStatus; 8] = [FloatStatus::CanFloat; 8];

pub unsafe extern "C" fn ganon_float(boma: *mut BattleObjectModuleAccessor) {
    if hash40("special_air_n") != MotionModule::motion_kind(boma) {
        return;
    }
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let motion_module_frame = MotionModule::frame(boma);
    println!(
        "Entry id {}: Original float state: {}",
        entry_id, FLOAT[entry_id]
    );
    FLOAT[entry_id] = match FLOAT[entry_id] {
        FloatStatus::CanFloat => {
            FLOAT[entry_id].transition_to_floating_if_able(&motion_module_frame)
        }
        FloatStatus::CannotFloat => FLOAT[entry_id].transition_to_can_float_if_able(
            &status_kind,
            situation_kind,
            smash::app::sv_information::is_ready_go(),
        ),
        FloatStatus::Floating(_) => FLOAT[entry_id].transition_to_cannot_float_if_able(
            &status_kind,
            ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL),
            check_jump(boma),
        ),
    };
    println!(
        "Entry id {}: New float state: {}",
        entry_id, FLOAT[entry_id]
    );
    match FLOAT[entry_id] {
        FloatStatus::CannotFloat => transition_to_fall(boma, &status_kind),
        FloatStatus::Floating(i) => {
            FLOAT[entry_id] = FloatStatus::Floating(i - 1);
        }
        _ => {}
    }
}
