//! Ideal workflow
//! FLOAT -> SPECIAL_AIR_S_CATCH -> SPECIAL_AIR_S_FALL -> SPECIAL_AIR_S_END
//!
//! Frame data for explosion
//! Frame 0 - 2 -> Small explosion, greatest knockback
//! Frame 3 - 6 -> Medium explosion medium knockback
//! Frame 7 - 13 -> Large explosion, least knockback
//! Frame 14 -> 18 -> meduium explosion, pathetic knockback
//! Frame 19 - 40 -> End lag.
use super::utils::*;
use skyline_smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash_script::macros;
use std::f32::consts::PI;
use {
    smash::{hash40, lua2cpp::*},
    smashline::*,
};

//
unsafe extern "C" fn precondition_check(
    boma: *mut BattleObjectModuleAccessor,
    iv: &InitValues,
) -> bool {
    if let FloatStatus::Floating(_) = GS[iv.entry_id].float_status {
        if !in_teleport(boma) && iv.status_kind == FIGHTER_STATUS_KIND_SPECIAL_LW {
            return true;
        }
    }
    false
}

pub unsafe extern "C" fn new_down_special(fighter: &mut L2CFighterCommon, iv: &InitValues) {
    let boma = fighter.module_accessor;
    let dive_check = in_dive(boma);
    if !dive_check {
        let pc_check = precondition_check(boma, iv);
        if pc_check {
            WorkModule::on_flag(boma, GANON_FLOAT_INTO_DIVE);
        } else {
            return;
        }
        return;
    }
    if iv.status_kind == FIGHTER_STATUS_KIND_SPECIAL_LW {
        GS[iv.entry_id].float_status = FloatStatus::CannotFloat;
        StatusModule::change_status_request_from_script(
            boma,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH,
            false.into(),
        );
    } else if iv.status_kind == FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL {
    } else if iv.status_kind == FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH {
    } else if iv.status_kind == FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END {
    } else {
        WorkModule::off_flag(boma, GANON_FLOAT_INTO_DIVE);
    }
}
