//! To simulate a 100% accurate Melee Marth, make Marth absolutely nasty in the air.

use crate::ganon::float_utils::ganon_float;
use skyline_smash::app::BattleObjectModuleAccessor;
use {
    smash::{app::lua_bind::*, hash40, lua2cpp::*},
    smashline::*,
};

pub unsafe extern "C" fn ganon_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    ganon_float(boma);
}

pub fn install() {
    Agent::new("ganon").on_line(Main, ganon_frame).install();
}
