use super::utils::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use {smash::lua2cpp::*, smashline::*};

// TODO:
//
// 1) If Status kind is equal to jump, set timer to 16 frames. Regardless of status kind,
//    change motion kind to float.
// 2) Do the same thing, as 1 but timer is set to 29.0 frames.
// 3) If double jump used, pressing the jump button will execute float automatically.
pub unsafe extern "C" fn float_check(fighter: &mut L2CFighterCommon, iv: &InitValues) {
    let boma = fighter.module_accessor;
    match GS[iv.entry_id].float_status {
        FloatStatus::CanFloat => {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) - 1
                != WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)
            {
                println!(
                    "Float Frame Counter: {}",
                    GS[iv.entry_id].pre_float_frame_counter,
                );
                if GS[iv.entry_id].pre_float_frame_counter == -1 {
                    if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP
                        || iv.status_kind == *FIGHTER_STATUS_KIND_CLIFF_JUMP2
                    {
                        GS[iv.entry_id].pre_float_frame_counter = 16;
                    } else if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                        GS[iv.entry_id].pre_float_frame_counter = 29;
                    }
                } else {
                    GS[iv.entry_id].pre_float_frame_counter -= 1;
                    if GS[iv.entry_id].pre_float_frame_counter == 0 {
                        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                            WorkModule::on_flag(boma, GANON_CAN_FLOAT_FLAG);
                        }
                    } else {
                    }
                }
            }
            // let frame_counter = WorkModule::get_int(boma, GANON_PRE_FLOAT_FRAME_COUNTER);
            // println!("Frame Counter: {}", frame_counter);
            // if frame_counter == -1 {
            //     if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP {
            //         WorkModule::set_int(boma, GANON_PRE_FLOAT_FRAME_COUNTER, 16);
            //     } else if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
            //         WorkModule::set_int(boma, GANON_PRE_FLOAT_FRAME_COUNTER, 29);
            //     }
            // } else {
            //     let frame_counter = frame_counter - 1;
            //     if frame_counter == 0 {
            //         if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
            //             WorkModule::on_flag(boma, GANON_CAN_FLOAT_FLAG);
            //         }
            //     }
            //     WorkModule::set_int(boma, GANON_PRE_FLOAT_FRAME_COUNTER, frame_counter);
            // }
        }
        _ => {
            WorkModule::off_flag(boma, GANON_CAN_FLOAT_FLAG);
        }
    }
}
//
// if GS[iv.entry_id].pre_float_frame_counter == 31 || StatusModule::is_changing(boma) {
//     if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP {
//         GS[iv.entry_id].pre_float_frame_counter = 16;
//     } else if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
//         GS[iv.entry_id].pre_float_frame_counter = 29;
//     }
// } else {
//     GS[iv.entry_id].pre_float_frame_counter -= 1;
//     if (GS[iv.entry_id].pre_float_frame_counter == 0
//         || WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT)
//             == WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
//         && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)
//     {
//         WorkModule::on_flag(boma, GANON_CAN_FLOAT_FLAG);
//     }
// }

// if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)
//     && ((iv.status_kind == *FIGHTER_STATUS_KIND_JUMP && iv.motion_module_frame == 16.0)
//         || (iv.status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL
//             && iv.motion_module_frame == 29.0))
// {
//     WorkModule::on_flag(boma, GANON_CAN_FLOAT_FLAG);
//     MotionModule::change_motion(
//         boma,
//         Hash40::new("jump_float"),
//         0.0,
//         1.0,
//         false,
//         0.0,
//         false,
//         false,
//     );
// }
