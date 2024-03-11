//! Given some input, this file calculates the next position to teleport to and handles
//! the starting logic.
use super::utils::*;
use smash::{
    app::{lua_bind::*, *},
    lib::lua_const::*,
    lua2cpp::*,
    phx::*,
};
use smash_script::*;

const MIN_TELEPORT_STEP: f32 = 20.0;
const MID_TELEPORT_STEP: f32 = 40.0;

/// Distance is measured by mash ferocity and direction of the left analog stick
fn calculate_base_teleport_distance(stick: f32) -> f32 {
    let stick_abs = stick.abs();
    let mut t_step = 0.0;
    if stick_abs > 0.2 && stick_abs <= 0.8 {
        t_step = MIN_TELEPORT_STEP;
    } else if stick_abs > 0.8 {
        t_step = MID_TELEPORT_STEP;
    }
    if stick < 0.0 {
        return -t_step;
    }
    t_step
}

/// Add teleport distance iff the
fn add_teleport_distance(direction: f32) -> f32 {
    if direction < 0.0 {
        return direction - 20.0;
    } else if direction > 0.0 {
        return direction + 20.0;
    }
    direction
}

unsafe extern "C" fn teleport_fx(fighter: &mut L2CFighterCommon) {
    macros::EFFECT(
        fighter,
        Hash40::new("ganon_entry"),
        Hash40::new("hip"),
        0,
        0,
        0,
        0,
        0,
        0,
        0.6,
        0,
        0,
        0,
        0,
        0,
        0,
        true,
    );
    macros::LAST_EFFECT_SET_RATE(fighter, 1.875); // 2.5 == 30 frames
    macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_l01"));
}

impl Position2D {
    // If on the ground, there are five different positions to choose from:
    // Up Right vertex:   x: 0.9166667, y: 0.5
    // Up Left vertex     x: -0.9166667, y: 0.5
    // Down Right vertex: x: 0.9166667, y: -0.5
    // Down Left vertex:  x: -0.9166667, y: -0.5
    unsafe extern "C" fn next_teleport_position(
        boma: *mut BattleObjectModuleAccessor,
        iv: &InitValues,
    ) -> Position2D {
        let mut x = calculate_base_teleport_distance(ControlModule::get_stick_x(boma));
        let mut y = calculate_base_teleport_distance(ControlModule::get_stick_y(boma));
        if y == 0.0 {
            x = add_teleport_distance(x);
        } else if x == 0.0 {
            y = add_teleport_distance(y);
        }
        if iv.situation_kind == SITUATION_KIND_GROUND && y < 0.0 {
            y = 0.0;
        }
        Position2D { x: x, y: y + 0.1 }
    }

    unsafe extern "C" fn set_to_work_module(self: &Self, boma: *mut BattleObjectModuleAccessor) {
        WorkModule::set_float(boma, self.x, GANON_TELEPORT_NEW_X_POS);
        WorkModule::set_float(boma, self.y, GANON_TELEPORT_NEW_Y_POS);
    }
}

pub unsafe extern "C" fn ganon_teleport_handler(fighter: &mut L2CFighterCommon, iv: &InitValues) {
    if iv.status_kind != FIGHTER_STATUS_KIND_SPECIAL_N && iv.situation_kind != SITUATION_KIND_GROUND
    {
        return;
    }
    let boma = fighter.module_accessor;
    let ts = TeleportStatus::from_int(WorkModule::get_int(boma, GANON_TELEPORT_WORK_INT));
    match ts {
        TeleportStatus::PreTransit => {
            Position2D::next_teleport_position(boma, iv).set_to_work_module(boma);
        }
        TeleportStatus::Transit => {
            PostureModule::add_pos_2d(
                boma,
                &Vector2f {
                    x: WorkModule::get_float(boma, GANON_TELEPORT_NEW_X_POS),
                    y: WorkModule::get_float(boma, GANON_TELEPORT_NEW_Y_POS),
                },
            );

            macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
            // VisibilityModule::set_whole(fighter.module_accessor, false);
            JostleModule::set_status(fighter.module_accessor, false);
            GroundModule::set_correct(
                fighter.module_accessor,
                GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
            );
            // teleport_fx(fighter);
            WorkModule::set_int(boma, TeleportStatus::End as i32, GANON_TELEPORT_WORK_INT);
            if !WorkModule::is_flag(boma, GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG) {
                WorkModule::set_flag(boma, true, GANON_TELEPORT_INTO_FLOAT_INIT_FLAG);
                WorkModule::set_int(boma, TeleportStatus::Ready as i32, GANON_TELEPORT_WORK_INT);
            }
        }
        _ => {}
    }
}
