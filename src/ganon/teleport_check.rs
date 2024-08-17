//! If Ganondorf used his teleport in the air, he cannot use it again until he lands
//! or gets hit with a strong move.
//! This file will ensure that we cannot abuse teleport.
use crate::imports::*;
use super::utils::*;

pub unsafe extern "C" fn teleport_check(fighter: &mut L2CFighterCommon, iv: &InitValues) {
    let boma = fighter.module_accessor;
    let can_teleport = WorkModule::is_flag(boma, GANON_CAN_TELEPORT_FLAG);
    if !can_teleport && (iv.situation_kind == *SITUATION_KIND_GROUND)
        || (iv.situation_kind == *SITUATION_KIND_AIR
            && [
                *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            ]
            .contains(&iv.status_kind))
    {
        WorkModule::on_flag(boma, GANON_CAN_TELEPORT_FLAG);
    } else if can_teleport
        && iv.situation_kind == *SITUATION_KIND_AIR
        && iv.status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
    {
        WorkModule::off_flag(boma, GANON_CAN_TELEPORT_FLAG);
    }
}
