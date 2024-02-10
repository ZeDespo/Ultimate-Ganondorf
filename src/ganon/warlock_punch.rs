//! Miscellaneous variables, utility functions, and others to help
//! facilitate critical functions to the mod.
use core::fmt;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::{hash40, lua2cpp::*};

use skyline_smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash_script::macros;
use smashline::*;

const WARLOCK_N_TURN_FRAMES: i16 = 127;

#[derive(Copy, Clone)]
enum WarlockMutex {
    Ready,
    Executing(i16),
}

static mut WARLOCK_ENTRIES: [WarlockMutex; 8] = [WarlockMutex::Ready; 8];

pub unsafe extern "C" fn warlock_punch(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    match WARLOCK_ENTRIES[entry_id] {
        WarlockMutex::Ready => {
            let status_kind = StatusModule::status_kind(boma);
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
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
                && !invalid_status_kinds.contains(&status_kind)
            {
                WARLOCK_ENTRIES[entry_id] = WarlockMutex::Executing(WARLOCK_N_TURN_FRAMES);
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                StatusModule::change_status_request_from_script(
                    boma,
                    *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN,
                    true,
                );
            }
        }
        WarlockMutex::Executing(0) => WARLOCK_ENTRIES[entry_id] = WarlockMutex::Ready,
        WarlockMutex::Executing(i) => WARLOCK_ENTRIES[entry_id] = WarlockMutex::Executing(i - 1),
    }
}
