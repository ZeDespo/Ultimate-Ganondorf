use super::utils::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use {smash::lua2cpp::*, smashline::*};

pub unsafe extern "C" fn float_check(fighter: &mut L2CFighterCommon, iv: &InitValues) {
    let boma = fighter.module_accessor;
    match GS[iv.entry_id].float_status {
        FloatStatus::CanFloat => {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)
                && ((iv.status_kind == *FIGHTER_STATUS_KIND_JUMP && iv.motion_module_frame == 16.0)
                    || (iv.status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL
                        && iv.motion_module_frame == 29.0))
            {
                WorkModule::on_flag(boma, GANON_CAN_FLOAT_FLAG);
                MotionModule::change_motion(
                    boma,
                    Hash40::new("jump_float"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false,
                );
            }
        }
        _ => WorkModule::off_flag(boma, GANON_CAN_FLOAT_FLAG),
    }
}
