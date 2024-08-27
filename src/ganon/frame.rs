use crate::imports::*;
use super::utils::*;
use crate::ganon::{
    float::ganon_float, float_check::float_check, new_down_special::new_down_special,
    teleport_check::teleport_check, warlock_punch::warlock_punch,
};

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
        start_float: WorkModule::is_flag(boma, GANON_START_FLOAT_FLAG),
        jump_button_pressed: ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP),
    };
    println!("{:#?}", iv);
    float_check(fighter, &iv);
    ganon_float(fighter, &iv);
    warlock_punch(fighter, &iv);
    new_down_special(fighter, &iv);
    teleport_check(fighter); // Removed InitValue
}

pub fn install() {
    Agent::new("ganon").on_line(Main, ganon_frame).install();
}
