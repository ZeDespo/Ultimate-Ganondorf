//! *NOTE*: This file is solely the work of the Ultimate S team, headed by `@chrispo`.
//!
//! The only credit I can claim is converting the library to use Smashline 2 and for
//! some Rust formatting.
use crate::ganon::utils::{TeleportStatus, GANON_TELEPORT_WORK_INT};
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::Hash40;
use smash_script::*;
use smashline::*;

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialn", ganon_teleport)
        .game_acmd("game_specialairn", ganon_float)
        .effect_acmd("effect_specialn", ganon_teleport_eff)
        .effect_acmd("effect_specialairn", ganon_floate)
        .sound_acmd("sound_specialn", ganon_teleport_snd)
        .sound_acmd("sound_specialairn", ganon_floats)
        .expression_acmd("expression_specialn", ganon_teleport_expr)
        .expression_acmd("expression_specialairn", ganon_float_expr)
        .install();
}

/// Take that base teleport from Ultimate S, but only yake the
unsafe extern "C" fn ganon_teleport(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(
            fighter.module_accessor,
            TeleportStatus::PreTransit.to_int(),
            GANON_TELEPORT_WORK_INT,
        );
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(
            fighter.module_accessor,
            TeleportStatus::Transit.to_int(),
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
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(
            fighter,
            Hash40::new("ganon_majinken_start"),
            Hash40::new("hip"),
            0,
            0,
            0,
            0,
            0,
            0,
            1.75,
            true,
        );
        macros::EFFECT_FOLLOW(
            fighter,
            Hash40::new("ganon_majinken_start"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1.0,
            true,
        );
        macros::EFFECT_FOLLOW(
            fighter,
            Hash40::new("ganon_majinken_start"),
            Hash40::new("havel"),
            0,
            0,
            0,
            0,
            0,
            0,
            1.0,
            true,
        );
        macros::EFFECT_FOLLOW(
            fighter,
            Hash40::new("ganon_majinken_start"),
            Hash40::new("footr"),
            0,
            0,
            0,
            0,
            0,
            0,
            1.0,
            true,
        );
        macros::EFFECT_FOLLOW(
            fighter,
            Hash40::new("ganon_majinken_start"),
            Hash40::new("footl"),
            0,
            0,
            0,
            0,
            0,
            0,
            1.0,
            true,
        );
    }
}

unsafe extern "C" fn ganon_teleport_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_l_tail"));
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_ganon_appeal_h01"));
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_l02"));
    }
}

unsafe extern "C" fn ganon_teleport_expr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
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
