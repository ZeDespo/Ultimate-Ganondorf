use crate::imports::*;

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attackdash", ganon_attackdash, Priority::Default)
        .effect_acmd(
            "effect_attackdash",
            ganon_effect_attackdash,
            Priority::Default,
        )
        .install();
}

unsafe extern "C" fn dash_attack_hitbox(agent: &mut L2CAgentBase, angle: u64, damage: f32) {
    let fkb1: i32;
    let fkb2: i32;
    let angle2: u64;
    if angle >= 360 {
        fkb1 = 5;
        fkb2 = 2;
        angle2 = 368;
    } else {
        fkb1 = 0;
        fkb2 = 0;
        angle2 = angle;
    }
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            damage,
            angle,
            82,
            fkb1,
            67,
            3.0,
            0.0,
            8.0,
            4.0,
            Some(0.0),
            Some(8.0),
            Some(8.5),
            0.5,
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
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_MAGIC,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            damage,
            angle2,
            82,
            fkb2,
            67,
            6.0,
            0.0,
            8.0,
            13.0,
            None,
            None,
            None,
            0.5,
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
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_MAGIC,
        );
        if angle2 == 368 {
            AttackModule::set_vec_target_pos(
                agent.module_accessor,
                1,
                Hash40::new("top"),
                &Vector2f { x: 1.0, y: 0.0 },
                4,
                false,
            );
        }
    }
}

// 16 - 22
// Hitbox on 16, 18, 20, with 22 hitting far way.
unsafe extern "C" fn ganon_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7286);
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    for _ in 0..3 {
        dash_attack_hitbox(agent, 363, 1.2);
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    dash_attack_hitbox(agent, 53, 7.7);
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    StatusModule::change_status_request_from_script(
        agent.module_accessor,
        *FIGHTER_STATUS_KIND_WAIT,
        false.into(),
    );
}

unsafe extern "C" fn ganon_effect_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS_NO_STOP(
            agent,
            Hash40::new("sys_thunder"),
            Hash40::new("handr"),
            2,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            -2,
            0,
            0,
            0,
            0,
            0,
            0.8,
            0,
            0,
            4,
            0,
            0,
            0,
            false,
        );
        macros::EFFECT_FLW_POS_NO_STOP(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("handr"),
            2,
            1,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_thunder_shot"),
            Hash40::new("top"),
            0.0,
            8.0,
            13.0,
            0,
            0,
            0,
            0.5,
            true,
        );
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_turn_smoke"),
            Hash40::new("top"),
            6,
            0,
            0,
            0,
            0,
            0,
            0.9,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_turn_smoke"),
            Hash40::new("top"),
            6,
            0,
            0,
            0,
            0,
            0,
            0.85,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
}
//
// unsafe extern "C" fn sound_attackdash(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 2.0);
//     if macros::is_excute(agent) {
//         macros::STOP_SE(agent, Hash40::new("se_edge_step_ll_loop"));
//         macros::PLAY_SE(agent, Hash40::new("se_edge_attackdash01"));
//     }
//     frame(agent.lua_state_agent, 13.0);
//     if macros::is_excute(agent) {
//         macros::PLAY_SE(agent, Hash40::new("se_edge_attackdash02"));
//     }
//     frame(agent.lua_state_agent, 20.0);
//     if macros::is_excute(agent) {
//         macros::PLAY_SEQUENCE(agent, Hash40::new("seq_edge_rnd_attack_hard"));
//     }
//     frame(agent.lua_state_agent, 36.0);
//     if macros::is_excute(agent) {
//         macros::PLAY_SE(agent, Hash40::new("se_edge_cloth01"));
//     }
//     frame(agent.lua_state_agent, 37.0);
//     if macros::is_excute(agent) {
//         macros::PLAY_SE(agent, Hash40::new("se_edge_step_right_s"));
//     }
// }
//
