//! This function just deals with the ground special attack's startup. How the rest of
//! the move is handled belongs to `crate::ganon::omni_teleport`.
use crate::ganon::utils::*;
use crate::imports::*;

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialhi", ganon_teleport, Priority::Default)
        .game_acmd("game_specialairhi", ganon_teleport, Priority::Default)
        .effect_acmd("effect_specialhi", ganon_teleport_eff, Priority::Default)
        .effect_acmd("effect_specialairhi", ganon_teleport_eff, Priority::Default)
        .sound_acmd("sound_specialhi", ganon_teleport_snd, Priority::Default)
        .sound_acmd("sound_specialairhi", ganon_teleport_snd, Priority::Default)
        .install();
}

/// Take that base teleport from Ultimate S, but only handle the entry animation.
/// Later handling is elsewhere.
unsafe extern "C" fn ganon_teleport(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, TELEPORT_TRANSIT_FRAME);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, TELEPORT_HITBOX_FRAME);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::ATTACK(
                agent,
                0,
                0,
                Hash40::new("top"),
                1.1,
                365,
                100,
                18,
                0,
                14.0,
                0.0,
                13.0,
                2.0,
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
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, TELEPORT_FRAMES - 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            7.2,
            90,
            108,
            0,
            23,
            14.0,
            0.0,
            13.0,
            2.0,
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
