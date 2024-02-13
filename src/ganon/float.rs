//! Miscellaneous variables, utility functions, and others to help
//! facilitate critical functions to the mod.
//!
//!
//!
//! 0x10 - 16 - SPECIAL_FALL
//! 0x12 - 18 - MOTION_AIR
//! 0x14 - 20 - MOTION_FALL
//!
use core::fmt;

use super::utils::*;
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

// TODO: This code has a weird quirk to it. Whenever an attack is thrown
// during a float while Ganondorf moves through the air, Ganondorf loses his ability
// to move in the last direction the attack was thrown in.
//
// On top of that, for the remainder of the float status. Ganondorf can move in the
// opposite direction at twice the normal speed he's allowed to.
//
// It's probably a misunderstanding of the physics engine, and I'd be more than happy to buff
// Ganon's float, but for now, will need to make this a feature.
const ATTACK_FRAME_LOSS: i16 = 30;

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

impl FloatStatus {
    fn transition_to_can_float_if_able(self: Self, init_values: &InitValues) -> FloatStatus {
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_DEAD,
        ]
        .contains(&init_values.status_kind)
            || init_values.situation_kind != SITUATION_KIND_AIR
        {
            return FloatStatus::CanFloat;
        }
        return self;
    }

    fn transition_to_cannot_float_if_able(
        self: Self,
        init_values: &InitValues,
        has_attacked: bool,
    ) -> FloatStatus {
        if let FloatStatus::Floating(i) = self {
            if i == 0
                || init_values.situation_kind != SITUATION_KIND_AIR
                || [
                    *FIGHTER_STATUS_KIND_JUMP_AERIAL,
                    *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                    *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                    *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
                ]
                .contains(&init_values.status_kind)
                || (init_values.status_kind == FIGHTER_STATUS_KIND_ATTACK_AIR && has_attacked)
            {
                return FloatStatus::CannotFloat;
            }
        }
        return self;
    }

    fn transition_to_floating_if_able(self: Self, init_values: &InitValues) -> FloatStatus {
        if init_values.motion_module_frame == STARTING_FLOAT_FRAME && init_values.is_special_air_n()
        {
            return FloatStatus::Floating(MAX_FLOAT_FRAMES);
        }
        return self;
    }
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
}

#[derive(Debug)]
struct InitValues {
    prev_status_kind: i32,
    status_kind: i32,
    situation_kind: i32,
    motion_kind: u64,
    entry_id: usize,
    motion_module_frame: f32,
}

impl InitValues {
    fn is_special_air_n(self: &Self) -> bool {
        self.motion_kind == hash40("special_air_n")
    }

    fn is_start_of_float(self: &Self) -> bool {
        self.motion_module_frame == STARTING_FLOAT_FRAME && self.is_special_air_n()
    }
}

pub unsafe extern "C" fn ganon_float(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let iv = InitValues {
        prev_status_kind: StatusModule::prev_status_kind(boma, 0),
        status_kind: StatusModule::status_kind(boma),
        situation_kind: StatusModule::situation_kind(boma),
        entry_id: WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize,
        motion_module_frame: MotionModule::frame(boma),
        motion_kind: MotionModule::motion_kind(boma),
    };
    println!("{:#?}", iv);
    println!("Original float state: {}", GS[iv.entry_id].fs);
    println!("Kinetic type: {}", KineticModule::get_kinetic_type(boma));
    GS[iv.entry_id].fs = match GS[iv.entry_id].fs {
        FloatStatus::CanFloat => GS[iv.entry_id].fs.transition_to_floating_if_able(&iv),
        FloatStatus::CannotFloat => GS[iv.entry_id].fs.transition_to_can_float_if_able(&iv),
        FloatStatus::Floating(_) => {
            let fs = GS[iv.entry_id]
                .fs
                .transition_to_cannot_float_if_able(&iv, GS[iv.entry_id].has_attacked);
            if matches!(fs, FloatStatus::Floating(_)) {
                GS[iv.entry_id].fs.transition_to_can_float_if_able(&iv)
            } else {
                fs
            }
        }
    };
    println!("New float state: {}", GS[iv.entry_id].fs);
    match GS[iv.entry_id].fs {
        FloatStatus::CannotFloat => {
            GS[iv.entry_id].speed = Speed::reset();
            GS[iv.entry_id].has_attacked = false;
            if iv.is_start_of_float() {
                StatusModule::change_status_request_from_script(
                    boma,
                    *FIGHTER_STATUS_KIND_FALL_AERIAL,
                    true,
                );
            } else if iv.status_kind == FIGHTER_STATUS_KIND_ATTACK_AIR {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
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
            if iv.prev_status_kind == FIGHTER_STATUS_KIND_ATTACK_AIR {
                let attack_frame_loss = i - ATTACK_FRAME_LOSS - 1;
                if !GS[iv.entry_id].has_attacked {
                    println!("New float i value: {}", attack_frame_loss);
                    if attack_frame_loss > 1 {
                        GS[iv.entry_id].fs = FloatStatus::Floating(attack_frame_loss);
                    } else {
                        GS[iv.entry_id].fs = FloatStatus::Floating(2);
                    }
                }
                GS[iv.entry_id].has_attacked = true;
            }
            if i - 1 == 0 {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            } else {
                let new_speed = GS[iv.entry_id].speed.calculate_new_speed(
                    ControlModule::get_stick_x(boma) * PostureModule::lr(boma),
                    ControlModule::get_stick_y(boma),
                );
                println!("Calculated new speed: {:#?}", new_speed);
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
