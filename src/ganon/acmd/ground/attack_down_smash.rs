use crate::imports::*;

unsafe extern "C" fn extended_hitbox_helper(
    agent: &mut L2CAgentBase,
    id: u64,
    x_offset: f32,
    y_offset: f32,
    z_offset: f32,
) {
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            id,
            0,
            Hash40::new("top"),
            8.0,
            90,
            100,
            0,
            40,
            5.0,
            x_offset,
            y_offset,
            z_offset,
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
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
    }
}

unsafe extern "C" fn ganon_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD,
        );
    }
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 0.85);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            2.0,
            368,
            100,
            90,
            0,
            5.0,
            0.0,
            0.0,
            8.0,
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
        AttackModule::set_vec_target_pos(
            agent.module_accessor,
            0,
            Hash40::new("top"),
            &Vector2f { x: -4.0, y: 7.0 },
            17,
            false,
        );
    }
    frame(agent.lua_state_agent, 25.0);
    macros::FT_MOTION_RATE(agent, 1.25);
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("handl"),
            14.0,
            361,
            100,
            0,
            60,
            7.0,
            0.0,
            0.0,
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

    frame(agent.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    extended_hitbox_helper(agent, 1, 0.0, 0.0, 8.0);
    extended_hitbox_helper(agent, 2, 0.0, 0.0, -8.0);
    frame(agent.lua_state_agent, 40.0);
    extended_hitbox_helper(agent, 3, 0.0, 0.0, 14.0);
    extended_hitbox_helper(agent, 4, 0.0, 0.0, -14.0);
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        AttackModule::clear(agent.module_accessor, 2, false);
    }
    extended_hitbox_helper(agent, 1, 0.0, 0.0, 24.0);
    extended_hitbox_helper(agent, 2, 0.0, 0.0, -24.0);
    frame(agent.lua_state_agent, 59.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attacklw4charge(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("havel"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.3);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("havel"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.3);
        }
        wait(agent.lua_state_agent, 10.0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
}

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    for _ in 0..7 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("havel"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.4);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("havel"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    for _ in 0..7 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("havel"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.6,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.4);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("havel"),
                0,
                0,
                0,
                2,
                0,
                0,
                0.6,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.4);
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("havel"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.6,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.4);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("havel"),
                0,
                0,
                0,
                -2,
                0,
                0,
                0.6,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        }
        wait(agent.lua_state_agent, 3.0);
        // macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
    wait(agent.lua_state_agent, 8.0);
    macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, false);
    macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            6,
            0,
            0,
            0,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.43);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            -4,
            0,
            0,
            0,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            6,
            0,
            0,
            2,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.43);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            -4,
            0,
            0,
            -2,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            10,
            0,
            0,
            0,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.43);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            -8,
            0,
            0,
            0,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            10,
            0,
            0,
            2,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.43);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            -8,
            0,
            0,
            -2,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            14,
            0,
            0,
            0,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.43);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            -12,
            0,
            0,
            0,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            14,
            0,
            0,
            2,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.43);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            -12,
            0,
            0,
            -2,
            0.62,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            18,
            0,
            0,
            0,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.43);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            -16,
            0,
            0,
            0,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            18,
            0,
            0,
            2,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.43);
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("top"),
            -4,
            2,
            -16,
            0,
            0,
            -2,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
    }
}

unsafe extern "C" fn expression_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ArticleModule::set_visibility_whole(
            agent.module_accessor,
            *FIGHTER_GANON_GENERATE_ARTICLE_SWORD,
            false,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(agent.lua_state_agent, 5.0);
    execute(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ArticleModule::set_visibility_whole(
            agent.module_accessor,
            *FIGHTER_GANON_GENERATE_ARTICLE_SWORD,
            false,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
        slope!(
            agent,
            *MA_MSC_CMD_SLOPE_SLOPE_INTP,
            *SLOPE_STATUS_TOP,
            2,
            true
        );
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attacklw4", ganon_attacklw4, Priority::Default)
        .effect_acmd("effect_attacklw4", effect_attacklw4, Priority::Default)
        .effect_acmd(
            "effect_attacklw4charge",
            effect_attacklw4charge,
            Priority::Default,
        )
        .expression_acmd(
            "expression_attacklw4",
            expression_attacklw4,
            Priority::Default,
        )
        .install();
}
