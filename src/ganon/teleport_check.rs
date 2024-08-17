//! If Ganondorf used his teleport in the air, he cannot use it again until he lands
//! or gets hit with a strong move.
//! This file will ensure that we cannot abuse teleport.
use crate::imports::*;
use super::utils::*;

pub unsafe extern "C" fn teleport_check(fighter: &mut L2CFighterCommon) {
    // Make sure to add the `&mut *`
    let boma = &mut *fighter.module_accessor;
    let can_teleport = WorkModule::is_flag(boma, GANON_CAN_TELEPORT_FLAG);

    if !can_teleport {
        if boma.is_situation(*SITUATION_KIND_GROUND)
        && [
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        ].contains(&boma.situation_kind()) {
            WorkModule::on_flag(boma, GANON_CAN_TELEPORT_FLAG);
        }
    } else if boma.is_situation(*SITUATION_KIND_AIR) && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        WorkModule::off_flag(boma, GANON_CAN_TELEPORT_FLAG);
    }
}
