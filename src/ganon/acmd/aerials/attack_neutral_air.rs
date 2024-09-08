use crate::ganon::utils::*;
use crate::imports::*;

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attackairn", ganon_attackairn, Priority::Default)
        .effect_acmd("effect_attackairn", effect_attackairn, Priority::Default)
        .sound_acmd("sound_attackairn", sound_attackairn, Priority::Default)
        .expression_acmd(
            "expression_attackairn",
            expression_attackairn,
            Priority::Default,
        )
        .install();
}

unsafe extern "C" fn ganon_attackairn(agent: &mut L2CAgentBase) {
    if !in_teleport(agent.module_accessor) {
        normal_nair(agent);
    } else {
        portal_hitbox(agent);
    }
}

unsafe extern "C" fn normal_nair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            8.5,
            361,
            72,
            0,
            64,
            12.5,
            0.0,
            10.0,
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
            Hash40::new("collision_attr_magic"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_MAGIC,
        );
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
}

unsafe extern "C" fn portal_hitbox(agent: &mut L2CAgentBase) {
    // macros::FT_MOTION_RATE(agent, 0.87879);
    frame(agent.lua_state_agent, 0.1);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..5 {
        if macros::is_excute(agent) {
            // macros::ATTACK(
            //     agent,
            //     0,
            //     0,
            //     Hash40::new("hip"),
            //     1.1,
            //     361,
            //     100,
            //     1,
            //     0,
            //     12.0,
            //     0.0,
            //     0.0,
            //     0.0,
            //     None,
            //     None,
            //     None,
            //     1.0,
            //     0.0,
            //     *ATTACK_SETOFF_KIND_OFF,
            //     *ATTACK_LR_CHECK_POS,
            //     false,
            //     0,
            //     0.0,
            //     0,
            //     false,
            //     false,
            //     false,
            //     false,
            //     true,
            //     *COLLISION_SITUATION_MASK_GA,
            //     *COLLISION_CATEGORY_MASK_ALL,
            //     *COLLISION_PART_MASK_ALL,
            //     false,
            //     Hash40::new("collision_attr_purple"),
            //     *ATTACK_SOUND_LEVEL_M,
            //     *COLLISION_SOUND_ATTR_ELEC,
            //     *ATTACK_REGION_NONE,
            // );
            // top is foot
            macros::ATTACK(
                agent,
                0,
                0,
                Hash40::new("top"),
                1.1,
                361,
                100,
                18,
                0,
                4.0,
                0.0,
                10.0,
                0.0,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_OFF,
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
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_NONE,
            );
            macros::ATTACK(
                agent,
                1,
                0,
                Hash40::new("top"),
                1.1,
                90,
                100,
                18,
                0,
                6.0,
                0.0,
                4.0,
                0.0,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_OFF,
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
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_NONE,
            );
            macros::ATTACK(
                agent,
                2,
                0,
                Hash40::new("top"),
                1.1,
                270,
                100,
                18,
                0,
                6.0,
                0.0,
                20.0,
                0.0,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_OFF,
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
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_NONE,
            );
            macros::ATTACK(
                agent,
                3,
                0,
                Hash40::new("hip"),
                1.1,
                45,
                100,
                18,
                0,
                6.0,
                2.5,
                -5.0,
                0.0,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_OFF,
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
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_NONE,
            );
            macros::ATTACK(
                agent,
                4,
                0,
                Hash40::new("hip"),
                1.1,
                180,
                100,
                18,
                0,
                6.0,
                -2.5,
                5.0,
                0.0,
                None,
                None,
                None,
                1.0,
                1.0,
                *ATTACK_SETOFF_KIND_OFF,
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
                *ATTACK_SOUND_LEVEL_M,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_NONE,
            );
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("hip"),
            7.0,
            90,
            108,
            0,
            23,
            12.0,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
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
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_NONE,
        );
    }
    macros::FT_MOTION_RATE(agent, 0.4);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    if !in_teleport(agent.module_accessor) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("ganon_final_hand_triforce"),
                Hash40::new("handr"),
                2.5,
                1,
                0,
                0,
                0,
                0,
                1,
                true,
            );
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_final_hand_triforce"), false, true);
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("ganon_raijin_bomb"),
                Hash40::new("top"),
                0,
                10,
                0,
                0,
                0,
                0,
                0.8,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 4.0);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_majinken_flash"), false, false);
        }
    }
}

unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    if !in_teleport(agent.module_accessor) {
        // frame(agent.lua_state_agent, 13.0);
        // if macros::is_excute(agent) {
        //     macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        // }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
        }
        // frame(agent.lua_state_agent, 15.0);
        // if macros::is_excute(agent) {
        //     macros::PLAY_SE(agent, Hash40::new("se_edge_attackair_n03"));
        // }
    }
}

unsafe extern "C" fn expression_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
}
