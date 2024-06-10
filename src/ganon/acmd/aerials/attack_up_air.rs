//! U-Air has an extra two frames of active hitboxes, from frame 14 to frame 18
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::Hash40;
use smash_script::*;
use smashline::*;

unsafe extern "C" fn ganon_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("legl"),
            13.0,
            361,
            100,
            0,
            35,
            4.8,
            3.2,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
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
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("kneel"),
            12.0,
            361,
            100,
            0,
            35,
            5.8,
            6.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
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
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("legl"),
            12.0,
            30,
            80,
            0,
            30,
            4.8,
            3.2,
            0.0,
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
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("kneel"),
            10.0,
            30,
            80,
            0,
            30,
            5.8,
            6.0,
            0.0,
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
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("legl"),
            8.0,
            0,
            70,
            0,
            20,
            4.8,
            3.2,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_B,
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
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("kneel"),
            6.0,
            0,
            70,
            0,
            20,
            5.8,
            6.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_B,
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
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
    }
    wait(agent.lua_state_agent, 5.0); // 2 extra frames of late u-air hitboxes.
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 57.0);
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
        1.0,
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
