//! Miscellaneous variables, utility functions, and others to help
//! facilitate critical functions to the mod.
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

const MAX_FLOAT_FRAMES: i16 = 90;
const STARTING_FLOAT_FRAME: f32 = 2.0;
const X_MAX: f32 = 1.155;
const X_ACCEL_MULT: f32 = 0.12;
const Y_MAX: f32 = X_MAX;
const Y_ACCEL_MULT: f32 = X_ACCEL_MULT;

unsafe extern "C" fn float_effect(fighter: &mut L2CFighterCommon) {
    macros::EFFECT_FOLLOW(
        fighter,
        Hash40::new("ganon_rekkikyaku"),
        Hash40::new("kneer"),
        12,
        -1.5,
        0,
        0,
        0,
        0,
        0.5,
        true,
    );
    macros::EFFECT_FOLLOW(
        fighter,
        Hash40::new("ganon_rekkikyaku"),
        Hash40::new("kneel"),
        12,
        -1.5,
        0,
        0,
        0,
        0,
        0.5,
        true,
    );
}

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
    // Floating status should be set when:
    // 1) Ganon is on the ground
    // 2) Ganon does a special move in the air
    // 3) Ganon loses a stock
    // 4) The match is over.
    fn transition_to_can_float_if_able(
        self: Self,
        init_values: &InitValues,
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
            .contains(&init_values.status_kind)
            {
                return FloatStatus::CanFloat;
            }
        }
        if init_values.situation_kind != SITUATION_KIND_AIR || !is_ready_go {
            return FloatStatus::CanFloat;
        }
        return self;
    }

    // Floating should be disabled when:
    // 1) Ganon floated for the maximum time allotted
    // 2) Ganon cancels the floating with another jump.
    // 3) Ganon performs an air dodge at any time.
    fn transition_to_cannot_float_if_able(self: Self, init_values: &InitValues) -> FloatStatus {
        match self {
            FloatStatus::Floating(i) => {
                if i == 0
                    || init_values.is_jump
                    || init_values.situation_kind != SITUATION_KIND_AIR
                    || [
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                    ]
                    .contains(&init_values.status_kind)
                {
                    return FloatStatus::CannotFloat;
                }
            }
            _ => {
                if [
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
                    *FIGHTER_STATUS_KIND_JUMP_AERIAL,
                ]
                .contains(&init_values.status_kind)
                {
                    return FloatStatus::CannotFloat;
                }
            }
        }
        return self;
    }

    fn transition_to_floating_if_able(self: Self, init_values: &InitValues) -> FloatStatus {
        if let FloatStatus::CanFloat = self {
            if init_values.motion_module_frame == STARTING_FLOAT_FRAME
                && init_values.is_special_air_n()
            {
                return FloatStatus::Floating(MAX_FLOAT_FRAMES);
            }
        }
        return self;
    }
}

#[derive(Copy, Clone, Debug)]
struct Speed {
    x: f32,
    y: f32,
}

impl Speed {
    fn calculate_new_speed(self: Self, stick_x: f32, stick_y: f32) -> Speed {
        let mut x_add = stick_x * X_ACCEL_MULT;
        let mut y_add = stick_y * Y_ACCEL_MULT;
        if (x_add > 0.0 && self.x > X_MAX) || (x_add < 0.0 && self.x < -X_MAX) {
            x_add = 0.0;
        }
        if (y_add > 0.0 && self.y > Y_MAX) || (y_add < 0.0 && self.y < -Y_MAX) {
            y_add = 0.0;
        }
        return Self { x: x_add, y: y_add };
    }

    fn reset() -> Speed {
        Speed { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug)]
struct InitValues {
    prev_status_kind: i32,
    status_kind: i32,
    situation_kind: i32,
    motion_kind: u64,
    entry_id: usize,
    motion_module_frame: f32,
    is_jump: bool,
}

impl InitValues {
    fn is_special_air_n(self: &Self) -> bool {
        self.motion_kind == hash40("special_air_n")
    }

    fn is_start_of_float(self: &Self) -> bool {
        self.motion_module_frame == STARTING_FLOAT_FRAME && self.is_special_air_n()
    }
}

#[derive(Copy, Clone)]
struct GanonState {
    fs: FloatStatus,
    speed: Speed,
    attack_1: bool,
}

static mut GS: [GanonState; 8] = [GanonState {
    fs: FloatStatus::CanFloat,
    speed: Speed { x: 0.0, y: 0.0 },
    attack_1: false,
}; 8];

pub unsafe extern "C" fn ganon_float(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let iv = InitValues {
        prev_status_kind: StatusModule::prev_status_kind(boma, 0),
        status_kind: StatusModule::status_kind(boma),
        situation_kind: StatusModule::situation_kind(boma),
        entry_id: WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize,
        motion_module_frame: MotionModule::frame(boma),
        motion_kind: MotionModule::motion_kind(boma),
        is_jump: ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP)
            || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)
            || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI),
    };
    println!("{:#?}", iv);
    println!("Original float state: {}", GS[iv.entry_id].fs);
    GS[iv.entry_id].fs = match GS[iv.entry_id].fs {
        FloatStatus::CanFloat => GS[iv.entry_id].fs.transition_to_floating_if_able(&iv),
        FloatStatus::CannotFloat => GS[iv.entry_id]
            .fs
            .transition_to_can_float_if_able(&iv, smash::app::sv_information::is_ready_go()),
        FloatStatus::Floating(_) => GS[iv.entry_id].fs.transition_to_cannot_float_if_able(&iv),
    };
    println!("New float state: {}", GS[iv.entry_id].fs);
    match GS[iv.entry_id].fs {
        FloatStatus::CannotFloat => {
            GS[iv.entry_id].speed = Speed::reset();
            if iv.is_start_of_float() {
                StatusModule::change_status_request_from_script(
                    boma,
                    *FIGHTER_STATUS_KIND_FALL_AERIAL,
                    true,
                );
            }
        }
        FloatStatus::Floating(i) => {
            println!("Current speed: {:#?}", GS[iv.entry_id].speed);
            if iv.is_start_of_float() {
                macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_l01"));
                CancelModule::enable_cancel(boma);
            }
            if i % 30 == 0 {
                float_effect(fighter);
            }
            GS[iv.entry_id].fs = FloatStatus::Floating(i - 1);
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            }
            if i - 1 == 0 {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            } else {
                let new_speed = GS[iv.entry_id].speed.calculate_new_speed(
                    ControlModule::get_stick_x(boma) * PostureModule::lr(boma),
                    ControlModule::get_stick_y(boma),
                );
                KineticModule::add_speed(
                    boma,
                    &smash::phx::Vector3f {
                        x: new_speed.x,
                        y: new_speed.y,
                        z: 0.0,
                    },
                );
                GS[iv.entry_id].speed = Speed {
                    x: GS[iv.entry_id].speed.x + new_speed.x,
                    y: GS[iv.entry_id].speed.y + new_speed.y,
                };
            }
            println!("New speed: {:#?}", GS[iv.entry_id].speed);
        }
        _ => GS[iv.entry_id].speed = Speed::reset(),
    }
}
