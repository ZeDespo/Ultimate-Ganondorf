use crate::ganon::utils::in_dive;
use crate::ganon::utils::FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT;
use crate::ganon::utils::GANON_DARK_RUPTURE_ACTIVE;
use crate::ganon::utils::GANON_FLOAT_INTO_DIVE;
use skyline_smash::app::BattleObjectModuleAccessor;
use skyline_smash::app::GroundCorrectKind;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::app::sv_battle_object::notify_event_msc_cmd;
use smash::lib::lua_const::*;
use smash_script::{damage, lua_args, macros, slope};
use {
    smash::{hash40, lua2cpp::*},
    smashline::*,
};

pub fn install() {
    Agent::new("ganon")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, teleport_init)
        .status(
            Init,
            FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
            teleport_calculator_init,
        )
        .status(
            Main,
            FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
            teleport_calculator_main,
        )
        .status(
            Exit,
            FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
            teleport_calculator_exit,
        )
        .install();
}

unsafe extern "C" fn teleport_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::change_status_request_from_script(
        fighter.module_accessor,
        FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
        false.into(),
    );
    0.into()
}

unsafe extern "C" fn teleport_calculator_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn teleport_calculator_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(
        teleport_calculator_main_loop as *const () as _,
    ))
}

unsafe extern "C" fn teleport_calculator_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::frame(fighter.module_accessor) == 5.0 {
        StatusModule::change_status_request_from_script(
            fighter.module_accessor,
            *FIGHTER_STATUS_KIND_DEAD,
            false.into(),
        );
    }
    0.into()
}

unsafe extern "C" fn teleport_calculator_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
