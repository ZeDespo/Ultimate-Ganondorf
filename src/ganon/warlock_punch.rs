//! Just because we replaced the neutral special doesn't mean we want to be rid of the
//! Warlock Punch.
//!
//! This file will bind mount the Warlock punch to other inputs.
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;

use super::utils::{FloatStatus, InitValues, GS};
use skyline_smash::app::BattleObjectModuleAccessor;
use smashline::*;

const WARLOCK_N_TURN_FRAMES: i16 = 127;

/// To determine state of Warlock Punch.
#[derive(Copy, Clone)]
enum WarlockMutex {
    Ready,
    Executing(i16),
}

static mut WARLOCK_ENTRIES: [WarlockMutex; 8] = [WarlockMutex::Ready; 8];

/// Position Ganondorf to perform the Warlock punch.
unsafe extern "C" fn execute_punch(boma: *mut BattleObjectModuleAccessor, direction: f32) {
    PostureModule::set_lr(boma, -direction); // Reverse for true direction
    PostureModule::update_rot_y_lr(boma);
    StatusModule::change_status_request_from_script(
        boma,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN,
        true,
    );
}

/// Determine which taunt button was pressed, if at all, and return a corresponding value.
unsafe extern "C" fn get_taunt_button_press(boma: *mut BattleObjectModuleAccessor) -> f32 {
    if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
        -1.0
    } else if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
        1.0
    } else {
        0.0
    }
}

/// Check if we are pressing the special button, to perform a neutral special attack.
unsafe extern "C" fn is_true_neutral_special(boma: *mut BattleObjectModuleAccessor) -> bool {
    ControlModule::get_stick_x(boma).abs() < 0.2
        && ControlModule::get_stick_y(boma).abs() < 0.2
        && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL)
}

/// Do the Warlock punch if any of the following conditions are met:
/// - The left taunt / right taunt button is pressed.
pub unsafe extern "C" fn warlock_punch(fighter: &mut L2CFighterCommon, iv: &InitValues) {
    let boma = fighter.module_accessor;
    match WARLOCK_ENTRIES[iv.entry_id] {
        WarlockMutex::Ready => {
            let invalid_status_kinds = [
                *FIGHTER_STATUS_KIND_ATTACK,
                *FIGHTER_STATUS_KIND_ATTACK_AIR,
                *FIGHTER_STATUS_KIND_FALL_SPECIAL,
                *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
            ];
            if !invalid_status_kinds.contains(&iv.status_kind) {
                let direction = get_taunt_button_press(boma);
                if direction != 0.0 {
                    WARLOCK_ENTRIES[iv.entry_id] = WarlockMutex::Executing(WARLOCK_N_TURN_FRAMES);
                    execute_punch(boma, direction);
                }
            }
        }
        WarlockMutex::Executing(0) => WARLOCK_ENTRIES[iv.entry_id] = WarlockMutex::Ready,
        WarlockMutex::Executing(i) => WARLOCK_ENTRIES[iv.entry_id] = WarlockMutex::Executing(i - 1),
    }
}
