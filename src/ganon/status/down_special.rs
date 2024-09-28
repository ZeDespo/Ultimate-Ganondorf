use crate::{
    ganon::utils::{
        BomaExt, GANON_DOWN_SPECIAL_AIR_CONTINUE_FLAG, GANON_DOWN_SPECIAL_AIR_COUNTDOWN_FLOAT,
        GANON_DOWN_SPECIAL_AIR_MULTIPLIER_FLAG,
    },
    imports::*,
};

unsafe extern "C" fn ganon_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::off_flag(
            fighter.module_accessor,
            GANON_DOWN_SPECIAL_AIR_MULTIPLIER_FLAG,
        );
        StatusModule::set_status_kind_interrupt(
            fighter.module_accessor,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH,
        );
        return 1.into();
    }

    original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

unsafe extern "C" fn ganon_special_air_s_fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(
        fighter.module_accessor,
        GANON_DOWN_SPECIAL_AIR_CONTINUE_FLAG,
    );
    WorkModule::set_int(
        fighter.module_accessor,
        -1,
        GANON_DOWN_SPECIAL_AIR_COUNTDOWN_FLOAT,
    );
    original_status(Pre, fighter, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL)(fighter)
}

unsafe extern "C" fn ganon_special_air_s_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let fvar6 = WorkModule::get_param_float(
        boma,
        0xfea97fe73,
        hash40("special_s_fall_check_dead_offset_y"),
    );
    WorkModule::set_float(
        boma,
        fvar6,
        *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y,
    );
    MotionModule::change_motion(
        boma,
        Hash40::new("special_air_s_fall"),
        0.0,
        1.0,
        true,
        0.0,
        false,
        false,
    );
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.sub_shift_status_main(L2CValue::Ptr(
        ganon_special_air_s_fall_main_loop as *const () as _,
    ))
}

unsafe extern "C" fn ganon_special_air_s_fall_main_loop(
    fighter: &mut L2CFighterCommon,
) -> L2CValue {
    let boma = &mut *fighter.module_accessor;
    let continue_fall = WorkModule::is_flag(boma, GANON_DOWN_SPECIAL_AIR_CONTINUE_FLAG);
    if continue_fall {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(
                FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(),
                false.into(),
            );
        }
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::off_flag(boma, GANON_DOWN_SPECIAL_AIR_CONTINUE_FLAG);
            WorkModule::set_int(boma, 25, GANON_DOWN_SPECIAL_AIR_COUNTDOWN_FLOAT);
            return 1.into();
        }
    } else {
        let countdown = WorkModule::get_int(boma, GANON_DOWN_SPECIAL_AIR_COUNTDOWN_FLOAT);
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if countdown <= 10 && countdown > 0 {
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
                        / 1.9
                );
            } else if countdown == 0 {
                if boma.is_situation(*SITUATION_KIND_AIR) {
                    KineticModule::clear_speed_all(boma);
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                } else {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                }
                return 0.into();
            }
            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                WorkModule::set_int(boma, countdown - 1, GANON_DOWN_SPECIAL_AIR_COUNTDOWN_FLOAT);
            }
        } else {
            if countdown <= 10 {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 1.into();
            } else {
                fighter.change_status(
                    FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(),
                    false.into(),
                );
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_speciallw_pre)
        .status(
            Pre,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL,
            ganon_special_air_s_fall_pre,
        )
        .status(
            Main,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL,
            ganon_special_air_s_fall_main,
        )
        .install();
}
