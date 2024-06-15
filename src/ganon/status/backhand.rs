use skyline_smash::app::GroundCliffCheckKind;
use skyline_smash::app::GroundCorrectKind;
use skyline_smash::app::SituationKind;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use {
    smash::{hash40, lua2cpp::*},
    smashline::*,
};

pub fn install() {
    Agent::new("ganon")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, stub)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, stub)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main)
        .install();
}

unsafe extern "C" fn stub(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(
        fighter.module_accessor,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
    );
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

pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int64(
        fighter.module_accessor,
        hash40("special_s") as i64,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND,
    );
    WorkModule::set_int64(
        fighter.module_accessor,
        hash40("special_air_s") as i64,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR,
    );
    if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter
            .sub_wait_ground_check_common(false.into())
            .get_bool()
            == false
        && fighter.sub_air_check_fall_common().get_bool()
    {
        return L2CValue::I32(0);
    }
    let motion_g = WorkModule::get_int64(
        fighter.module_accessor,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND,
    );
    let motion_a = WorkModule::get_int64(
        fighter.module_accessor,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR,
    );
    fighter.sub_change_motion_by_situation(
        Hash40::new_raw(motion_g).into(),
        Hash40::new_raw(motion_a).into(),
        true.into(),
    );
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(
            fighter.module_accessor,
            GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
        );
    } else {
        GroundModule::correct(
            fighter.module_accessor,
            GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP),
        );
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    return L2CValue::I32(0);
}
