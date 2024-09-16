//! D-Air can be cancelled on frame 30 instead of frame 32.
use crate::imports::*;

const DAIR_LENGTH: f32 = 30.0;
const DAIR_FRAME: f32 = 17.0;

unsafe extern "C" fn ganon_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
        JostleModule::set_status(agent.module_accessor, false);
    }
    macros::FT_MOTION_RATE(agent, 0.7675);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            5.0,
            82,
            100,
            0,
            40,
            4.0,
            0.0,
            4.0,
            4.0,
            Some(0.0),
            Some(-28.0),
            Some(4.0),
            1.2,
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
            false,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_ENERGY,
        );
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            14.0,
            280,
            78,
            0,
            32,
            6.0,
            0.0,
            -28.0,
            4.0,
            None,
            None,
            None,
            1.4,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
            false,
            4,
            0.0,
            0,
            false,
            false,
            false,
            false,
            false,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_ENERGY,
        );
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(
            agent.module_accessor,
            4.5,
            4.5,
            12.5,
            11.0,
        );
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
        JostleModule::set_status(agent.module_accessor, true);
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
    frame(agent.lua_state_agent, 16.0);
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
