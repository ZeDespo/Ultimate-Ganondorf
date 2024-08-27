use crate::imports::*;
use crate::ganon::utils::*;
use crate::utils::shield::*;

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialsbackhand", game_backhand, Priority::Default)
        .game_acmd("game_specialairsbackhand", game_backhand, Priority::Default)
        .effect_acmd("effect_specialsbackhand", effect_backhand, Priority::Default)
        .effect_acmd("effect_specialairsbackhand", effect_backhand, Priority::Default)
        .sound_acmd("sound_specialsbackhand", sound_backhand, Priority::Default)
        .sound_acmd("sound_specialairsbackhand", sound_airbackhand, Priority::Default)
        .expression_acmd("expression_specialsbackhand", expression_backhand, Priority::Default)
        .expression_acmd("expression_specialairsbackhand", expression_airbackhand, Priority::Default)
        .install();
}

unsafe extern "C" fn game_backhand(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn effect_backhand(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn sound_backhand(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ganon_special_s02"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s03"));
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_special_s01"));
    }
    wait(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

unsafe extern "C" fn sound_airbackhand(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_special_s01"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_ganon_special_s01"));
        macros::STOP_SE(agent, Hash40::new("se_ganon_special_s02"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_s03"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
}

unsafe extern "C" fn expression_backhand(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn expression_airbackhand(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
