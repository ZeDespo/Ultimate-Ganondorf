//! This function is for the ground/landing hitbox for piledrive
use crate::{ganon::utils::GANON_DOWN_SPECIAL_AIR_DURATION_FLAG, imports::*};

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    let increase_power = WorkModule::is_flag(agent.module_accessor, GANON_DOWN_SPECIAL_AIR_DURATION_FLAG);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("handl"),
            20.0,
            361,
            100,
            0,
            40,
            5.5,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
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
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_MAGIC,
        );
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("handl"),
            12.0,
            73,
            80,
            0,
            20,
            10.0,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
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
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_MAGIC,
        );
    }
    frame(agent.lua_state_agent, 7.0);
    let size = if increase_power {48.0} else {18.0};
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("handl"),
            8.0,
            45,
            60,
            0,
            20,
            size,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
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
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_MAGIC,
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialairs", game_specialairs, Priority::Default)
        .install();
}
