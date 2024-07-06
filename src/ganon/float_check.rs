use super::utils::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use {smash::lua2cpp::*, smashline::*};

pub unsafe extern "C" fn float_check(fighter: &mut L2CFighterCommon, iv: &InitValues) {
    let boma = fighter.module_accessor;
    // if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)
    //     && iv.motion_module_frame == 8.0
    //     && iv.situation_kind == *SITUATION_KIND_AIR
    if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_N
        && iv.situation_kind == *SITUATION_KIND_AIR
    {
        StatusModule::change_status_request_from_script(
            boma,
            FIGHTER_GANON_STATUS_KIND_FLOAT,
            false,
        );
        MotionModule::change_motion_kind(boma, Hash40::new("jump_float"));
    }
}
