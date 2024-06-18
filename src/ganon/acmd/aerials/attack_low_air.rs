//! D-Air can be cancelled on frame 30 instead of frame 32.
use std::convert::TryInto;

use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::Hash40;
use smash_script::*;
use smashline::*;

const DAIR_LENGTH: f32 = 30.0;
const DAIR_FRAME: f32 = 17.0;

unsafe extern "C" fn ganon_attackairlw(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 1.0);
    // if macros::is_excute(agent) {
    //     FighterAreaModuleImpl::enable_fix_jostle_area_xy(
    //         agent.module_accessor,
    //         4.5,
    //         4.5,
    //         12.5,
    //         0.0,
    //     );
    // }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, DAIR_FRAME);
    let step: u64 = 5;
    if macros::is_excute(agent) {
        for i in (0..(DAIR_LENGTH as u64) + 1).step_by(step as usize) {
            let y_dist = i as f32;
            let damage: f32;
            let angle: u64;
            let kbg: i32;
            let bkb: i32;
            let hitlag: f32;
            if y_dist < DAIR_LENGTH {
                (damage, angle, kbg, bkb, hitlag) = (8.2, 90, 60, 60, 0.9);
            } else {
                (damage, angle, kbg, bkb, hitlag) = (17.0, 270, 78, 97, 1.5);
            }
            let id: u64;
            if i == 0 {
                id = 0
            } else {
                id = (i / step);
            }
            macros::ATTACK(
                agent,
                id,
                0,
                Hash40::new("top"),
                damage,
                angle,
                kbg,
                0,
                bkb,
                2.5,
                17.0,
                -y_dist,
                10.0,
                None,
                None,
                None,
                hitlag,
                1.0,
                *ATTACK_SETOFF_KIND_ON,
                *ATTACK_LR_CHECK_POS,
                false,
                3,
                0.0,
                0,
                false,
                false,
                false,
                false,
                true,
                *COLLISION_SITUATION_MASK_G,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_normal"),
                *ATTACK_SOUND_LEVEL_L,
                *COLLISION_SOUND_ATTR_ELEC,
                *ATTACK_REGION_MAGIC,
            );
        }
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0); // Formerly 32.0
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
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
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("trail_thunder_bullet"),
            Hash40::new("top"),
            0,
            -35,
            4,
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
        macros::EFFECT(
            agent,
            Hash40::new("trail_thunder_shot"),
            Hash40::new("top"),
            0,
            -35,
            4,
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
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_thunder_bullet"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_thunder_shot"), false, false);
    }
}

unsafe extern "C" fn sound_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_ll"));
    }
    frame(agent.lua_state_agent, DAIR_FRAME);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_electric_hit_l"));
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attackairlw", ganon_attackairlw, Priority::Default)
        .effect_acmd("effect_attackairlw", effect_attackairlw, Priority::Default)
        .sound_acmd("sound_attackairlw", sound_attackairlw, Priority::Default)
        .install();
}
