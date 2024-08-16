//! U-Air has an extra two frames of active hitboxes, from frame 14 to frame 18
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::{Hash40, Vector2f};
use smash_script::*;
use smashline::*;

unsafe extern "C" fn portal_hitbox(
    agent: &mut L2CAgentBase,
    angle: u64,
    damage: f32,
    fixed_knockback: i32,
    base_knockback: i32,
) {
    macros::ATTACK(
        agent,
        0,
        0,
        Hash40::new("throw"),
        damage,
        angle,
        55,
        fixed_knockback,
        base_knockback,
        12.0,
        3.2,
        0.0,
        0.0,
        None,
        None,
        None,
        0.78,
        1.0,
        *ATTACK_SETOFF_KIND_OFF,
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
        *ATTACK_SOUND_LEVEL_M,
        *COLLISION_SOUND_ATTR_FIRE,
        *ATTACK_REGION_MAGIC,
    );
}

unsafe extern "C" fn ganon_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 8.0);
    for i in 0..4 {
        if macros::is_excute(agent) {
            if i == 0 {
                macros::ATTACK(
                    agent,
                    1,
                    0,
                    Hash40::new("handr"),
                    0.8,
                    368,
                    100,
                    120,
                    0,
                    5.0,
                    8.0,
                    0.0,
                    0.0,
                    None,
                    None,
                    None,
                    1.0,
                    1.0,
                    *ATTACK_SETOFF_KIND_OFF,
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
                    *ATTACK_SOUND_LEVEL_M,
                    *COLLISION_SOUND_ATTR_PUNCH,
                    *ATTACK_REGION_MAGIC,
                );
                macros::ATTACK(
                    agent,
                    2,
                    0,
                    Hash40::new("handr"),
                    0.8,
                    368,
                    100,
                    120,
                    0,
                    5.0,
                    0.0,
                    0.0,
                    0.0,
                    None,
                    None,
                    None,
                    1.0,
                    1.0,
                    *ATTACK_SETOFF_KIND_OFF,
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
                    *ATTACK_SOUND_LEVEL_M,
                    *COLLISION_SOUND_ATTR_PUNCH,
                    *ATTACK_REGION_MAGIC,
                );
                AttackModule::set_vec_target_pos(
                    agent.module_accessor,
                    1,
                    Hash40::new("top"),
                    &Vector2f { x: 10.0, y: 25.0 },
                    8,
                    false,
                );
                AttackModule::set_vec_target_pos(
                    agent.module_accessor,
                    2,
                    Hash40::new("top"),
                    &Vector2f { x: 0.0, y: 25.0 },
                    8,
                    false,
                );
            }
            portal_hitbox(agent, 367, 1.6, 35, 0);
        }
        wait(agent.lua_state_agent, 4.0);
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        portal_hitbox(agent, 270, 12.0, 0, 25);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(
            agent,
            Hash40::new_raw(0x2127e37c07),
            *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES
        );
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    macros::EFFECT_FOLLOW(
        agent,
        Hash40::new("ganon_entry"),
        Hash40::new("throw"),
        0,
        0,
        0,
        0,
        0,
        0,
        0.5,
        true,
    );
    macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 12.0);
    for _ in 0..12 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("ganon_attack_purple"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                1.0,
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
    }
    wait(agent.lua_state_agent, 1.0);
    for _ in 0..12 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("ganon_attack_purple"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.25,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 2.2);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_purple"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                1.2,
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
        wait(agent.lua_state_agent, 1.0);
    }
    wait(agent.lua_state_agent, 1.0);
    macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_entry"), false, false);
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attackairhi", ganon_attackairhi, Priority::Default)
        .effect_acmd("effect_attackairhi", effect_attackairhi, Priority::Default)
        .install();
}
