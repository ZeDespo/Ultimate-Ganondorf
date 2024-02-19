use crate::ganon::float::ganon_float;
use crate::ganon::omni_teleport::ganon_teleport_handler;
use crate::ganon::warlock_punch::warlock_punch;
use {smash::lua2cpp::*, smashline::*};

pub unsafe extern "C" fn ganon_frame(fighter: &mut L2CFighterCommon) {
    ganon_teleport_handler(fighter); //
    ganon_float(fighter);
    warlock_punch(fighter);
}

pub fn install() {
    Agent::new("ganon").on_line(Main, ganon_frame).install();
}
