//!
//! Return 0 => Keep going with the status kind scripts.
//! Return 1 => Do not continue with the status kind.
//!
//! Pre => constructor
//! Init => Post constructor (not necessary)
//! Main > MainLoop => Logic regarding current execution
//! Exec => Runs logic one frame after main (not necessary)
//! End => Status script gracefully ends
//! Exit => Status script has been interrupted by something (like a hit) (not necessary)
//!
//! set_status_kind_interrupt is goated for status scripts to transition from one SK to another.
use crate::imports::*;
use crate::ganon::utils::*;

pub fn install() {
    Agent::new("ganon")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, transition_to_backhand)
        .status(Pre, FIGHTER_GANON_STATUS_KIND_BACKHAND, specials_pre)
        .status(Main, FIGHTER_GANON_STATUS_KIND_BACKHAND, special_s_main)
        .status(End, FIGHTER_GANON_STATUS_KIND_BACKHAND, specials_end)
        .install();
}

unsafe extern "C" fn transition_to_backhand(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(
        fighter.module_accessor,
        FIGHTER_GANON_STATUS_KIND_BACKHAND,
    );
    1.into()
}

unsafe extern "C" fn specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        0,
        0,
    );
    return L2CValue::I32(1);
}

/// Perform first time checks for certain animations, initializing some things that
/// don't quite belong in Init, such as resetting flags and changing animations,
pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // The following if/else statement is to check if we did the animation for the first
    // time, as we need to call _SLIGHTLY_ different functions.
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        GroundModule::correct(
            fighter.module_accessor,
            GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP),
        );
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s_backhand"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false,
        );
    } else {
        GroundModule::correct(
            fighter.module_accessor,
            GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
        );
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s_backhand"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false,
        );
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter
            .sub_wait_ground_check_common(false.into())
            .get_bool()
            == false
        && fighter.sub_air_check_fall_common().get_bool()
    {
        // Handles cancels to transition out of the status given certain unique
        // situations
        return L2CValue::I32(0);
    }
    if StatusModule::is_changing(boma) || StatusModule::is_situation_changed(boma) {
        // If you do a move in the air then land on the ground, we want to ensure
        // the aerial animation rules are still followed, despite using a grounded
        // animation. This is mostly for visual purposes that has no impact on
        // the hitboxes.
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(
                fighter.module_accessor,
                GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP),
            );
            KineticModule::change_kinetic(
                fighter.module_accessor,
                *FIGHTER_KINETIC_TYPE_GROUND_STOP,
            );
            MotionModule::change_motion_inherit_frame(
                boma,
                Hash40::new("special_s_backhand"),
                -1.0,
                1.0,
                0.0,
                false,
                false,
            );
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(
                fighter.module_accessor,
                GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
            );
            MotionModule::change_motion_inherit_frame(
                boma,
                Hash40::new("special_air_s_backhand"),
                -1.0,
                1.0,
                0.0,
                false,
                false,
            );
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}
