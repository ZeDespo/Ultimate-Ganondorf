use crate::imports::*;
use crate::ganon::utils::*;

pub unsafe extern "C" fn down_air_stall(fighter: &mut L2CFighterCommon) {
    let boma = &mut *fighter.module_accessor;

    if boma.is_situation(*SITUATION_KIND_GROUND)
    || boma.is_situation(*SITUATION_KIND_CLIFF)
    || StopModule::is_damage(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, GANON_DOWN_AIR_STALL_FLAG);
    }
}
