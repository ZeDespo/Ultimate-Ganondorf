use crate::imports::*;
use crate::ganon::utils::*;
use crate::utils::shield::*;

pub fn install() {
    Agent::new("ganon")
        .game_acmd(
            "game_specialairsfall",
            ganon_specialairsfall,
            Priority::Default,
        )
        .game_acmd("game_specials", ganon_specials, Priority::Default)
        .game_acmd("game_specialairs", ganon_specials, Priority::Default)
        .effect_acmd("effect_specials", effect_attacks3, Priority::Default)
        .effect_acmd("effect_specialairs", effect_attacks3, Priority::Default)
        .install();
}

unsafe extern "C" fn ganon_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("handl"),
            14.5,
            88,
            5,
            0,
            102,
            7.0,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            0.3,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            5,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_G,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_paralyze"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_PUNCH,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("handl"),
            14.0,
            81,
            5,
            0,
            92,
            7.0,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            0.3,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            5,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_A,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_paralyze"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_PUNCH,
        );
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1.2, 5.0);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 19.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 14.0, false);
        AttackModule::set_attack_camera_quake_forced(
            agent.module_accessor,
            0,
            *CAMERA_QUAKE_KIND_L,
            false,
        );
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(
            agent,
            *MA_MSC_CMD_REFLECTOR,
            *COLLISION_KIND_REFLECTOR,
            0,
            Hash40::new("top"),
            13.0,
            0.0,
            12.0,
            1.0,
            0.0,
            4.0,
            0.0,
            1.0,
            1.2,
            80,
            false,
            1.0,
            *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT
        );
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        shield!(
            agent,
            *MA_MSC_CMD_SHIELD_OFF,
            *COLLISION_KIND_REFLECTOR,
            0,
            *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT
        );
    }
}

// Backhand movements.
unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(
            agent,
            Hash40::new("mario_supermant_wind_r"),
            Hash40::new("mario_supermant_wind_l"),
            Hash40::new("top"),
            2.5,
            4,
            5,
            0,
            0,
            0,
            2.5,
            true,
            *EF_FLIP_NONE,
        );
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(
            agent,
            Hash40::new("sys_whirlwind_r"),
            Hash40::new("sys_whirlwind_l"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
            *EF_FLIP_NONE,
        );
    }
}

unsafe extern "C" fn ganon_specialairsfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(
            agent,
            *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,
            0,
            4.0,
            0,
            10,
            0,
            100,
            0.0,
            1.0,
            *ATTACK_LR_CHECK_F,
            0.0,
            true,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_NONE,
        );
        macros::SET_SPEED_EX(agent, 0, -5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(
            agent.module_accessor,
            *FIGHTER_GANON_EXPLOSION_FALL_SETTING_FALL,
            *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING,
        );
        if in_dive(agent.module_accessor) {
            macros::ATTACK(
                agent,
                1,
                0,
                Hash40::new("handl"),
                14.0,
                80,
                100,
                0,
                50,
                3.5,
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
                10,
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
                Hash40::new("collision_attr_purple"),
                *ATTACK_SOUND_LEVEL_L,
                *COLLISION_SOUND_ATTR_PUNCH,
                *ATTACK_REGION_PUNCH,
            );
        }
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
