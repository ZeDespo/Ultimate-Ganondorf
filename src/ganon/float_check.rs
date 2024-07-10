use super::utils::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;

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
            println!(
                "Float Activation Status: {}",
                GS[iv.entry_id].float_activation_status
            );
            match GS[iv.entry_id].float_activation_status {
                FloatActivationStatus::Waiting => {
                    if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP
                        || iv.status_kind == *FIGHTER_STATUS_KIND_CLIFF_JUMP2
                    {
                        GS[iv.entry_id].float_activation_status = FloatActivationStatus::Jump(16);
                    } else if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                        GS[iv.entry_id].float_activation_status =
                            FloatActivationStatus::JumpAerial(29);
                    }
                }
                FloatActivationStatus::Jump(i) => {
                    let frame_counter = i - 1;
                    if frame_counter == 0 {
                        if iv.jump_button_pressed {
                            WorkModule::on_flag(boma, GANON_START_FLOAT_FLAG);
                        } else {
                            GS[iv.entry_id].float_activation_status =
                                FloatActivationStatus::JumpUsed;
                        }
                    } else {
                        GS[iv.entry_id].float_activation_status =
                            FloatActivationStatus::Jump(frame_counter);
                    }
                }
                FloatActivationStatus::JumpUsed => {
                    if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                        GS[iv.entry_id].float_activation_status =
                            FloatActivationStatus::JumpAerial(29);
                    } else if iv.situation_kind == *SITUATION_KIND_GROUND {
                        GS[iv.entry_id].float_activation_status = FloatActivationStatus::Waiting;
                    }
                }
                FloatActivationStatus::JumpAerial(i) => {
                    let frame_counter = i - 1;
                    if frame_counter == 0 {
                        if iv.jump_button_pressed {
                            WorkModule::on_flag(boma, GANON_START_FLOAT_FLAG);
                        } else {
                            GS[iv.entry_id].float_activation_status =
                                FloatActivationStatus::JumpAerialUsed;
                        }
                    } else {
                        GS[iv.entry_id].float_activation_status =
                            FloatActivationStatus::JumpAerial(frame_counter);
                    }
                }
                FloatActivationStatus::JumpAerialUsed => {
                    if iv.jump_button_pressed {
                        WorkModule::on_flag(boma, GANON_START_FLOAT_FLAG);
                        GS[iv.entry_id].float_activation_status =
                            FloatActivationStatus::NotApplicable;
                    }
                    if iv.situation_kind == *SITUATION_KIND_GROUND {
                        GS[iv.entry_id].float_activation_status = FloatActivationStatus::Waiting;
                    }
                }
                FloatActivationStatus::NotApplicable => {}
            }
            //
            // if iv.situation_kind == *SITUATION_KIND_GROUND {
            //     GS[iv.entry_id].pre_float_frame_counter = -1;
            // }
            // println!(
            //     "Float Frame Counter: {}",
            //     GS[iv.entry_id].pre_float_frame_counter,
            // );
            // if GS[iv.entry_id].pre_float_frame_counter == -1 {
            //     if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP
            //         || iv.status_kind == *FIGHTER_STATUS_KIND_CLIFF_JUMP2
            //     {
            //         GS[iv.entry_id].pre_float_frame_counter = 16;
            //     } else if iv.status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
            //         GS[iv.entry_id].pre_float_frame_counter = 29;
            //     }
            // } else if GS[iv.entry_id].pre_float_frame_counter != -2 {
            //     GS[iv.entry_id].pre_float_frame_counter -= 1;
            //     if GS[iv.entry_id].pre_float_frame_counter == 0 {
            //         if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
            //             WorkModule::on_flag(boma, GANON_START_FLOAT_FLAG);
            //         } else {
            //             GS[iv.entry_id].pre_float_frame_counter = -2;
            //         }
            //     }
            // }
            // if GS[iv.entry_id].pre_float_frame_counter == -2
            //     && !WorkModule::is_flag(boma, GANON_PRE_FLOAT_MUTEX)
            //     && ![*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL]
            //         .contains(&iv.status_kind)
            // {
            //     if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
            //         WorkModule::on_flag(boma, GANON_START_FLOAT_FLAG);
            //         WorkModule::on_flag(boma, GANON_PRE_FLOAT_MUTEX);
            //     }
            // }
            //
            //
            //
            //
            //
            //
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
            WorkModule::off_flag(boma, GANON_START_FLOAT_FLAG);
            WorkModule::off_flag(boma, GANON_PRE_FLOAT_MUTEX);
            GS[iv.entry_id].float_activation_status = FloatActivationStatus::Waiting;
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
