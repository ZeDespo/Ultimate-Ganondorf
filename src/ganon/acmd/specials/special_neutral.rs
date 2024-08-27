//! This function just deals with the ground special attack's startup. How the rest of
//! the move is handled belongs to `crate::ganon::omni_teleport`.
use crate::imports::*;
use crate::ganon::utils::*;

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialn", ganon_teleport, Priority::Default)
        .game_acmd("game_specialairn", ganon_float, Priority::Default)
        .effect_acmd("effect_specialn", ganon_teleport_eff, Priority::Default)
        .effect_acmd("effect_specialairn", ganon_floate, Priority::Default)
        .sound_acmd("sound_specialn", ganon_teleport_snd, Priority::Default)
        .sound_acmd("sound_specialairn", ganon_floats, Priority::Default)
        .expression_acmd(
            "expression_specialairn",
            ganon_float_expr,
            Priority::Default,
        )
        .game_acmd("game_float_ganon", acmd_stub, Priority::Default)
        .effect_acmd("effect_float", acmd_stub, Priority::Default)
        .sound_acmd("sound_float", acmd_stub, Priority::Default)
        .expression_acmd("expression_float", acmd_stub, Priority::Default)
        .install();
}

/// Take that base teleport from Ultimate S, but only handle the entry animation.
/// Later handling is elsewhere.
unsafe extern "C" fn ganon_teleport(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(
            fighter.module_accessor,
            TeleportStatus::PreTransit as i32,
            GANON_TELEPORT_WORK_INT,
        );
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(
            fighter.module_accessor,
            TeleportStatus::Transit as i32,
            GANON_TELEPORT_WORK_INT,
        );
    }
}

unsafe extern "C" fn ganon_teleport_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..6 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(
                fighter,
                Hash40::new("ganon_entry_aura"),
                Hash40::new("emit"),
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                true,
            )
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(
            fighter,
            Hash40::new("ganon_entry"),
            Hash40::new("hip"),
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
        macros::LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(
            fighter,
            Hash40::new("ganon_entry"),
            Hash40::new("hip"),
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
        macros::LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
}

unsafe extern "C" fn ganon_teleport_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_appeal_h01"));
    }
}

// The following functions are to unload the default sounds / effects.
unsafe extern "C" fn ganon_float(fighter: &mut L2CAgentBase) {
    let _lua_state = fighter.lua_state_agent;
}

unsafe extern "C" fn ganon_floats(fighter: &mut L2CAgentBase) {
    let _lua_state = fighter.lua_state_agent;
}

unsafe extern "C" fn ganon_floate(fighter: &mut L2CAgentBase) {
    let _lua_state = fighter.lua_state_agent;
}

unsafe extern "C" fn ganon_float_expr(fighter: &mut L2CAgentBase) {
    let _lua_state = fighter.lua_state_agent;
}
