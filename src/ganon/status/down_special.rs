use crate::{ganon::utils::GANON_DOWN_SPECIAL_AIR_DURATION_FLAG, imports::*};

unsafe extern "C" fn ganon_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::off_flag(fighter.module_accessor, GANON_DOWN_SPECIAL_AIR_DURATION_FLAG);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH);
        return 1.into();
    }

    original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub fn install() {
    Agent::new("ganon")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_speciallw_pre)
        .install();
}
