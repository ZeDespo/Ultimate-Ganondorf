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

static WARLOCK_APPEAL: [u64; 2] = [hash40("appeal_hi_l"), hash40("appeal_hi_r")];

pub unsafe extern "C" fn warlock_punch(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    if WARLOCK_APPEAL.contains(&MotionModule::motion_kind(boma)) {
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
        StatusModule::change_status_request_from_script(
            boma,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN,
            true,
        );
    }
}