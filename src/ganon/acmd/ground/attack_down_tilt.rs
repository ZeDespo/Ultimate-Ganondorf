use crate::ganon::utils::{BomaExt, GANON_DOWN_TILT_2_FLAG};
use crate::imports::*;

/// Down tilt part 1 attack hitboxes.
unsafe extern "C" fn ganon_down_tilt_1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("handl"),
            7.1,
            270,
            100,
            1,
            35,
            5.0,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_MAGIC,
        );
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

/// Down tilt part 2 attack hitboxes.
unsafe extern "C" fn ganon_down_tilt_2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("handr"),
            9.1,
            90,
            100,
            1,
            35,
            6.0,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            3.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.2,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_stab"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_MAGIC,
        );
    }
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    wait(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, GANON_DOWN_TILT_2_FLAG)
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(
            agent.module_accessor,
            *FIGHTER_STATUS_KIND_WAIT,
            false.into(),
        );
    }
}

unsafe extern "C" fn ganon_down_tilt(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, GANON_DOWN_TILT_2_FLAG) {
        ganon_down_tilt_1(agent);
    } else {
        ganon_down_tilt_2(agent);
    }
}

unsafe extern "C" fn ganon_down_tilt_expression(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, GANON_DOWN_TILT_2_FLAG) {
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        }
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attacklw3", ganon_down_tilt, Priority::Default)
        .expression_acmd(
            "expression_attacklw3",
            ganon_down_tilt_expression,
            Priority::Default,
        )
        .install();
}
