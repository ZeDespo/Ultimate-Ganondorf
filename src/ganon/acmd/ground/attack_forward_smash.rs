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
unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    for _ in 0..4 {
        if macros::is_excute(agent) {
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
                0.4,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.4,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("throw"),
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
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
        wait(agent.lua_state_agent, 3.0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
    frame(agent.lua_state_agent, 16.0);
    for _ in 0..7 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("throw"),
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
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("throw"),
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
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("throw"),
                0,
                0,
                -1,
                0,
                5,
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
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("throw"),
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
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
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
        }
        wait(agent.lua_state_agent, 1.0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("throw"),
            0,
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
            true,
        );
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("throw"),
            0,
            0,
            0,
            2,
            3,
            5,
            0.95,
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
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
}

unsafe extern "C" fn effect_attacks4charge(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("throw"),
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
                Hash40::new("throw"),
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
                Hash40::new("sys_thunder"),
                Hash40::new("haver"),
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
                Hash40::new("sys_thunder"),
                Hash40::new("haver"),
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
    }
}

pub fn install() {
    Agent::new("ganon")
        .effect_acmd("effect_attacks4", effect_attacks4, Priority::Default)
        .effect_acmd(
            "effect_attacks4charge",
            effect_attacks4charge,
            Priority::Default,
        )
        .install();
}
