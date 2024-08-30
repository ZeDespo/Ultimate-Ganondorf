use super::utils::*;
use crate::imports::*;

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
                        if !iv.jump_button_pressed {
                            GS[iv.entry_id].float_activation_status =
                                FloatActivationStatus::JumpUsed
                        } else {
                            GS[iv.entry_id].float_activation_status =
                                FloatActivationStatus::Jump(frame_counter);
                        }
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
                    if iv.jump_button_pressed
                        && iv.status_kind != *FIGHTER_STATUS_KIND_CLIFF_CATCH
                        && iv.status_kind != *FIGHTER_STATUS_KIND_CLIFF_WAIT
                        && iv.status_kind != *FIGHTER_STATUS_KIND_CLIFF_JUMP1
                        && iv.status_kind != *FIGHTER_STATUS_KIND_CLIFF_ROBBED
                    {
                        println!("I FUCKED UP");
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
        }
        _ => {
            WorkModule::off_flag(boma, GANON_START_FLOAT_FLAG);
            WorkModule::off_flag(boma, GANON_PRE_FLOAT_MUTEX);
            GS[iv.entry_id].float_activation_status = FloatActivationStatus::Waiting;
        }
    }
}
