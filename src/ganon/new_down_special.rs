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
use {smash::lua2cpp::*, smashline::*};

pub unsafe extern "C" fn new_down_special(fighter: &mut L2CFighterCommon, iv: &InitValues) {
    let boma = fighter.module_accessor;
    if !in_dive(boma) {
        if iv.status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if iv.situation_kind == *SITUATION_KIND_GROUND {
                WorkModule::on_flag(boma, GANON_DOWN_SPECIAL_GROUND);
            } else if iv.situation_kind == *SITUATION_KIND_AIR
                && !WorkModule::is_flag(boma, GANON_DOWN_SPECIAL_GROUND)
            {
                StatusModule::change_status_request_from_script(
                    boma,
                    *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH,
                    false.into(),
                );
            }
        }
    }
}
