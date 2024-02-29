use super::utils::{in_teleport, InitValues, GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG};
use crate::ganon::float::ganon_float;
use crate::ganon::omni_teleport::ganon_teleport_handler;
use crate::ganon::warlock_punch::warlock_punch;
use skyline_smash::lib::lua_const::FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID;
use smash::app::lua_bind::*;
use {smash::lua2cpp::*, smashline::*};

pub unsafe extern "C" fn ganon_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let iv = InitValues {
        prev_status_kind: StatusModule::prev_status_kind(boma, 0),
        status_kind: StatusModule::status_kind(boma),
        situation_kind: StatusModule::situation_kind(boma),
        entry_id: WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize,
        motion_module_frame: MotionModule::frame(boma),
        motion_kind: MotionModule::motion_kind(boma),
        kinetic_kind: KineticModule::get_kinetic_type(boma),
        teleport_into_float: in_teleport(boma),
    };
    println!("{:#?}", iv);
    ganon_teleport_handler(fighter, &iv); //
    ganon_float(fighter, &iv);
    warlock_punch(fighter, &iv);
}

pub fn install() {
    Agent::new("ganon").on_line(Main, ganon_frame).install();
}
