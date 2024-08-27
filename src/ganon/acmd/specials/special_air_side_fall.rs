//! This function is for piledrive, which is performed by doing down b in the air
use crate::imports::*;

unsafe extern "C" fn ganon_specialairsfall(agent: &mut L2CAgentBase) {
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
        macros::SET_SPEED_EX(agent, 0, -5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_GANON_EXPLOSION_FALL_SETTING_FALL, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
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
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("handl"),
            8.0,
            45,
            60,
            0,
            20,
            18.0,
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

unsafe extern "C" fn effect_specialairsfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("ganon_engokua_catch"),
            Hash40::new("havel"),
            -1,
            0,
            0.5,
            0,
            0,
            0,
            1,
            true,
        );
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialairsfall", ganon_specialairsfall, Priority::Default)
        .game_acmd("effect_specialairsfall", effect_specialairsfall, Priority::Default)
        .install();
}
