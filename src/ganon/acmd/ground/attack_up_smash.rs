use skyline_smash::app::ArticleOperationTarget;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::phx::Vector2f;
use smash_script::macros;
use {smash::lua2cpp::*, smashline::*};

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attackhi4", ganon_attackhi4)
        .install();
}

unsafe extern "C" fn ganon_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(
            agent.module_accessor,
            *FIGHTER_GANON_GENERATE_ARTICLE_SWORD,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD,
        );
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("top"),
            2.0,
            368,
            100,
            90,
            0,
            3.0,
            0.0,
            11.7,
            -6.0,
            Some(0.0),
            Some(11.7),
            Some(7.5),
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
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
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
        AttackModule::set_vec_target_pos(
            agent.module_accessor,
            2,
            Hash40::new("top"),
            &Vector2f { x: 0.0, y: 24.0 },
            7,
            false,
        );
    }
    frame(agent.lua_state_agent, 10.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::ATTACK(
                agent,
                0,
                0,
                Hash40::new("top"),
                2.0,
                170,
                100,
                18,
                0,
                4.6,
                0.0,
                22.0,
                -5.8,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_THRU,
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
                Hash40::new("collision_attr_purple"),
                *ATTACK_SOUND_LEVEL_S,
                *COLLISION_SOUND_ATTR_FIRE,
                *ATTACK_REGION_NONE,
            );
            macros::ATTACK(
                agent,
                1,
                0,
                Hash40::new("top"),
                2.0,
                170,
                100,
                18,
                0,
                4.6,
                0.0,
                22.0,
                5.8,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_THRU,
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
                Hash40::new("collision_attr_purple"),
                *ATTACK_SOUND_LEVEL_S,
                *COLLISION_SOUND_ATTR_FIRE,
                *ATTACK_REGION_NONE,
            );
            macros::ATTACK(
                agent,
                2,
                0,
                Hash40::new("top"),
                2.0,
                95,
                100,
                40,
                0,
                3.5,
                0.0,
                14.0,
                0.0,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_THRU,
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
                *COLLISION_SITUATION_MASK_A,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_purple"),
                *ATTACK_SOUND_LEVEL_S,
                *COLLISION_SOUND_ATTR_FIRE,
                *ATTACK_REGION_NONE,
            );
            macros::ATTACK(
                agent,
                3,
                0,
                Hash40::new("top"),
                2.0,
                270,
                100,
                5,
                0,
                4.0,
                0.0,
                23.6,
                0.0,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_THRU,
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
                Hash40::new("collision_attr_purple"),
                *ATTACK_SOUND_LEVEL_S,
                *COLLISION_SOUND_ATTR_FIRE,
                *ATTACK_REGION_NONE,
            );
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            10.0,
            80,
            111,
            0,
            60,
            6.4,
            0.0,
            22.0,
            -7.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
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
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            10.0,
            80,
            111,
            0,
            60,
            6.4,
            0.0,
            22.0,
            7.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
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
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("top"),
            10.0,
            90,
            111,
            0,
            60,
            5.2,
            0.0,
            23.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
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
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
        macros::ATTACK(
            agent,
            3,
            0,
            Hash40::new("top"),
            10.0,
            90,
            111,
            0,
            60,
            4.0,
            0.0,
            14.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
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
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
