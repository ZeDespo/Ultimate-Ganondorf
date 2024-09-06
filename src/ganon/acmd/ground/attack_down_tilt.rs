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

/// Down tilt part 1 attack hitboxes.
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
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, GANON_DOWN_TILT_2_FLAG)
    }
}

unsafe extern "C" fn ganon_down_tilt(agent: &mut L2CAgentBase) {
    if !WorkModule::is_flag(agent.module_accessor, GANON_DOWN_TILT_2_FLAG) {
        ganon_down_tilt_1(agent);
    } else {
        ganon_down_tilt_2(agent);
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attacklw3", ganon_down_tilt, Priority::Default)
        .install();
}
