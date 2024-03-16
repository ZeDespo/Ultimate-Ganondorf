//! New Down Special
//!
//! Ganondorf plunges into the ground, causing an explosion.
//!
use crate::ganon::utils::in_dive;
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

const SITUATION_KIND: i32 = 0x16;

unsafe extern "C" fn fun_7100010b20(agent: &mut L2CFighterCommon) {
    agent.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(
        agent.module_accessor,
        GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND),
    );
}

unsafe extern "C" fn fun_7100006ef0(
    agent: &mut L2CFighterCommon,
    param_2: L2CValue,
    param_3: L2CValue,
) {
    let al_stack80: u64 = 0x32e468d950;
    agent.clear_lua_stack();
    agent.push_lua_stack(&mut L2CValue::new_int(al_stack80));
    agent.push_lua_stack(&mut L2CValue::new_int(param_2.get_u64()));
    agent.push_lua_stack(&mut L2CValue::new_int(param_3.get_u64()));
    notify_event_msc_cmd(agent.lua_state_agent);
    agent.pop_lua_stack(1);
}

unsafe extern "C" fn ganon_specialairsend_init(agent: &mut L2CFighterCommon) -> L2CValue {
    fun_7100006ef0(
        agent,
        hash40("catched_ganon").into(),
        hash40("catched_air_end_ganon").into(),
    );
    0.into()
}

unsafe extern "C" fn ganon_specialairsend_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let h_var3 = Hash40::new_raw(0xd2b3a620b); // special_air_s
    let f_var4 = 0.0;
    let f_var5 = 1.0;
    let b_var1 = false;
    MotionModule::change_motion(
        agent.module_accessor,
        h_var3,
        f_var4,
        f_var5,
        b_var1,
        0.0,
        false,
        false,
    );
    fun_7100010b20(agent);
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    agent.sub_shift_status_main(L2CValue::Ptr(
        ganon_specialarsend_main_loop as *const () as _,
    ))
}

unsafe extern "C" fn ganon_specialarsend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !CancelModule::is_enable_cancel(boma) {
        let curr_situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        if curr_situation_kind != *SITUATION_KIND_GROUND {
            if curr_situation_kind != *SITUATION_KIND_AIR {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        } else {
            if !MotionModule::is_end(boma) {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    } else {
        if fighter
            .sub_wait_ground_check_common(L2CValue::Bool(false))
            .get_bool()
            && fighter.sub_air_check_fall_common().get_bool()
        {
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn ganon_specialairsend_exit(agent: &mut L2CFighterCommon) -> L2CValue {
    CatchModule::catch_cut(agent.module_accessor, false, false);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
        .status(
            Init,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
            ganon_specialairsend_init,
        )
        .status(
            Main,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
            ganon_specialairsend_main,
        )
        .status(
            Exit,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
            ganon_specialairsend_exit,
        )
        .install();
}
