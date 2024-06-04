use skyline_smash::app::ArticleOperationTarget;

use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

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
