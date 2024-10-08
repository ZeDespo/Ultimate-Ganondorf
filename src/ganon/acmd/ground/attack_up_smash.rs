use crate::ganon::utils::*;
use crate::imports::*;

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attackhi4", ganon_attackhi4, Priority::Default)
        .effect_acmd(
            "effect_attackhi4charge",
            effect_attackhi4charge,
            Priority::Default,
        )
        .effect_acmd("effect_attackhi4", effect_attackhi4, Priority::Default)
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
            3.3,
            0.0,
            11.7,
            -13.0,
            Some(0.0),
            Some(11.7),
            Some(15.2),
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
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
        AttackModule::set_vec_target_pos(
            agent.module_accessor,
            2,
            Hash40::new("top"),
            &Vector2f { x: 0.0, y: 20.0 },
            7,
            false,
        );
    }
    frame(agent.lua_state_agent, 15.0);
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
                Hash40::new("collision_attr_elec"),
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
                Hash40::new("collision_attr_elec"),
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
                Hash40::new("collision_attr_elec"),
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
                Hash40::new("collision_attr_elec"),
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
            Hash40::new("collision_attr_elec"),
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
            Hash40::new("collision_attr_elec"),
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
            Hash40::new("collision_attr_elec"),
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
            Hash40::new("collision_attr_elec"),
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

unsafe extern "C" fn effect_attackhi4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_run_smoke"),
            Hash40::new("top"),
            -4,
            0,
            0,
            0,
            0,
            0,
            1,
            15,
            0,
            10,
            0,
            0,
            0,
            false,
        );
    }
    wait(agent.lua_state_agent, 5.0);
    macros::EFFECT(
        agent,
        Hash40::new("sys_smash_flash_s"),
        Hash40::new("haver"),
        0,
        16,
        0,
        0,
        0,
        0,
        1,
        4,
        4,
        4,
        0,
        0,
        0,
        true,
    );
}

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("haver"),
            0,
            18,
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
        );
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("younglink_final_triforce"),
            Hash40::new("top"),
            0,
            1,
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
        );
        macros::EFFECT(
            agent,
            Hash40::new("reflet_entry"),
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
        );
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_quake"),
            Hash40::new("top"),
            1.5,
            0,
            -18,
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
        );
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_landing_smoke"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            1.1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }

    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("younglink_final_triforce"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_entry"), false, false);
    }
}
