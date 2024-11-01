use crate::imports::*;

// Middle point for up-tilt geyser.
const MEDIAN: f32 = 22.0;

unsafe extern "C" fn ground_hitbox(
    agent: &mut L2CAgentBase,
    id: u64,
    angle: u64,
    z_offset: f32,
    y_offset: f32,
) {
    macros::ATTACK(
        agent,
        id,
        0,
        Hash40::new("top"),
        0.8,
        angle,
        100,
        10,
        35,
        5.0,
        0.0,
        y_offset,
        z_offset,
        None,
        None,
        None,
        0.7,
        1.0,
        *ATTACK_SETOFF_KIND_OFF,
        *ATTACK_LR_CHECK_F,
        false,
        0,
        0.0,
        1,
        false,
        false,
        false,
        false,
        true,
        *COLLISION_SITUATION_MASK_GA,
        *COLLISION_CATEGORY_MASK_ALL,
        *COLLISION_PART_MASK_ALL,
        false,
        Hash40::new("collision_attr_normal"),
        *ATTACK_SOUND_LEVEL_M,
        *COLLISION_SOUND_ATTR_FIRE,
        *ATTACK_REGION_MAGIC,
    );
    if angle == 368 {
        let autolink_x_offset: f32;
        let autolink_y_offset: f32;
        if z_offset < MEDIAN {
            autolink_x_offset = 6.0;
            autolink_y_offset = 10.0;
        } else if z_offset > MEDIAN {
            autolink_x_offset = -6.0;
            autolink_y_offset = 10.0;
        } else {
            autolink_x_offset = 0.0;
            autolink_y_offset = 5.0;
        }
        AttackModule::set_vec_target_pos(
            agent.module_accessor,
            id as i32,
            Hash40::new("top"),
            &Vector2f {
                x: autolink_x_offset,
                y: autolink_y_offset,
            },
            5,
            false,
        );
    }
}

unsafe extern "C" fn ganon_utilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ground_hitbox(fighter, 0, 70, MEDIAN - 8.0, 2.0);
        ground_hitbox(fighter, 1, 90, MEDIAN, 4.0);
        ground_hitbox(fighter, 2, 368, MEDIAN + 8.0, 2.0);
    }
    frame(fighter.lua_state_agent, 11.0);
    for _ in 0..2 {
        ground_hitbox(fighter, 3, 90, MEDIAN, 12.0);
        ground_hitbox(fighter, 4, 365, MEDIAN, 20.0);
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 3, false);
            AttackModule::clear(fighter.module_accessor, 4, false);
            // AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            3,
            0,
            Hash40::new("top"),
            7.8,
            75,
            100,
            0,
            45,
            8.0,
            0.0,
            22.0,
            MEDIAN,
            None,
            None,
            None,
            0.7,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            1,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_MAGIC,
        );
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn ganon_utilt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    for _ in 0..6 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_purple"),
                Hash40::new("havel"),
                0,
                1,
                0,
                0,
                0,
                0,
                0.7,
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
                Hash40::new("ganon_rekkikyaku"),
                Hash40::new("top"),
                0,
                4,
                MEDIAN,
                0,
                0,
                -90,
                0.8,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
            macros::LAST_EFFECT_SET_SCALE_W(agent, 0.8, 1.6, 0.5);
            macros::EFFECT(
                agent,
                Hash40::new("ganon_entry"),
                Hash40::new("top"),
                4,
                0,
                MEDIAN - 6.0,
                0,
                0,
                0,
                0.3,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.9);
            macros::EFFECT(
                agent,
                Hash40::new("ganon_entry"),
                Hash40::new("top"),
                -4,
                0,
                MEDIAN - 6.0,
                0,
                0,
                0,
                0.3,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.9);
            macros::EFFECT(
                agent,
                Hash40::new("ganon_entry"),
                Hash40::new("top"),
                6,
                0,
                MEDIAN,
                0,
                0,
                0,
                0.3,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.9);
            macros::EFFECT(
                agent,
                Hash40::new("ganon_entry"),
                Hash40::new("top"),
                -6,
                0,
                MEDIAN,
                0,
                0,
                0,
                0.3,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.9);
            macros::EFFECT(
                agent,
                Hash40::new("ganon_entry"),
                Hash40::new("top"),
                4,
                0,
                MEDIAN + 6.0,
                0,
                0,
                0,
                0.3,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.9);
            macros::EFFECT(
                agent,
                Hash40::new("ganon_entry"),
                Hash40::new("top"),
                -4,
                0,
                MEDIAN + 6.0,
                0,
                0,
                0,
                0.3,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.9);
        }
        wait(agent.lua_state_agent, 3.0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_rekkikyaku"), false, false);
        // macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry"), false, false);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_purple"),
            Hash40::new("havel"),
            0,
            1,
            0,
            0,
            0,
            0,
            0.7,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    wait(agent.lua_state_agent, 3.0);
    macros::EFFECT(
        agent,
        Hash40::new("ganon_rekkikyaku"),
        Hash40::new("top"),
        0,
        4,
        MEDIAN,
        0,
        0,
        -90,
        0.7,
        0,
        0,
        0,
        0,
        0,
        0,
        true,
    );
    macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    macros::LAST_EFFECT_SET_SCALE_W(agent, 0.8, 1.6, 0.5);
}

unsafe extern "C" fn ganon_utilt_expr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}

unsafe extern "C" fn ganon_utilt_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_attackhard_h02"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_ganon_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_attackhard_h03"));
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attackhi3", ganon_utilt, Priority::Default)
        .effect_acmd("effect_attackhi3", ganon_utilt_eff, Priority::Default)
        .sound_acmd("sound_attackhi3", ganon_utilt_snd, Priority::Default)
        .expression_acmd("expression_attackhi3", ganon_utilt_expr, Priority::Default)
        .install();
}
