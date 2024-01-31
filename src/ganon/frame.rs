use crate::ganon::float_utils::ganon_float;
use crate::ganon::warlock_punch_on_taunt::warlock_punch;
use {
    smash::{app::lua_bind::*, hash40, lua2cpp::*},
    smashline::*,
};

pub unsafe extern "C" fn ganon_frame(fighter: &mut L2CFighterCommon) {
    ganon_float(fighter);
    warlock_punch(fighter);
}

pub fn install() {
    Agent::new("ganon").on_line(Main, ganon_frame).install();
}
