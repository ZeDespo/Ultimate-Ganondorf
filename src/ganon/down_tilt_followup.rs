use crate::ganon::utils::{BomaExt, GANON_DOWN_TILT_2_FLAG};
use crate::imports::*;
// If in the down tilt animation and want to attack again, transition into the
// second crouch attack.

pub unsafe extern "C" fn down_tilt_followup_input_checker(fighter: &mut L2CFighterCommon) {
    let boma = &mut *fighter.module_accessor;
    if boma.motion_kind() == hash40("attack_lw3")
        && boma.motion_module_frame() >= 12.0
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)
        && ControlModule::get_stick_y(boma) < -0.8
    {
        WorkModule::on_flag(boma, GANON_DOWN_TILT_2_FLAG);
        MotionModule::change_motion(
            boma,
            Hash40::new("attack_lw32"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false,
        );
    }
}
