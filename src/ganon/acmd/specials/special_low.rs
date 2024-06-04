//! - On frame 16 until frame 39, a reflector hitbox will spawn on Ganondorf's foot, eating
//! MOST projectiles that come his way. Fast projectiles such as Byleth's fully charged
//! Failnaught arrow will be reflected back at the opponent.
//! - Tapping the special button will yield a shorter Wizard's Kick animation ending on frame 24.
//!    - Reflector will only last until frame 24 as well.
//! - Holding the special button past frame 10 will use the old Wizard's Kick animation.

use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash_script::macros;
use {smash::lua2cpp::*, smashline::*};

use crate::ganon::utils::FloatStatus;
use crate::ganon::utils::GS;
use crate::utils::shield::*;

unsafe extern "C" fn ganon_speciallw(agent: &mut L2CAgentBase) {
    let mut cancel_special = true;
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 3.0, 6.0, 8.5, 9.5);
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            cancel_special = false;
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(
            agent.module_accessor,
            2.0,
            6.0,
            8.5,
            10.0,
        );
    }
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 0.95);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("kneer"),
            14.0,
            45,
            65,
            0,
            65,
            4.0,
            2.7,
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
            4,
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
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("kneer"),
            16.0,
            45,
            65,
            0,
            65,
            5.0,
            7.0,
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
            4,
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
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        activate_reflector(
            agent,
            0,
            Hash40::new("kneer"),
            6.0,
            2.7,
            2.0,
            0.0,
            2.7,
            2.0,
            0.0,
            2.0,
            1.2,
            200,
            0.01,
        );
        JostleModule::set_status(agent.module_accessor, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK,
        );
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        if cancel_special {
            disable_reflector(agent, 0);
            StatusModule::change_status_request_from_script(
                agent.module_accessor,
                *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END,
                true,
            );
        }
    }
    frame(agent.lua_state_agent, 38.0); // Formerly 35.0, adjusted for new motion rate.
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 8.0, 8.0, 8.0, 4.0);
    }
    frame(agent.lua_state_agent, 39.0); // Formerly 36.0, adjusted for new motion rate
    if macros::is_excute(agent) {
        disable_reflector(agent, 0);
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn ganon_specialairlw(agent: &mut L2CAgentBase) {
    // StatusModule::change_status_request_from_script(
    //     agent.module_accessor,
    //     *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL,
    //     false.into(),
    // );
    // return;
    let mut cancel_special = true;
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            cancel_special = false;
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK,
        );
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("legl"),
            15.0,
            290,
            100,
            0,
            50,
            5.0,
            12.0,
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
            10,
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
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("legl"),
            15.0,
            290,
            100,
            0,
            50,
            3.5,
            8.0,
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
            10,
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
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        activate_reflector(
            agent,
            0,
            Hash40::new("kneer"),
            8.0,
            2.7,
            2.0,
            0.0,
            2.7,
            2.0,
            0.0,
            2.0,
            1.2,
            200,
            0.01,
        );
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("legl"),
            14.0,
            80,
            100,
            0,
            50,
            5.0,
            12.0,
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
            10,
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
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("legl"),
            14.0,
            80,
            100,
            0,
            50,
            3.5,
            8.0,
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
            10,
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
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_KICK,
        );
    }
    frame(agent.lua_state_agent, 27.0); // W/O motion rate, should be about frame 24.0
    if macros::is_excute(agent) {
        if cancel_special {
            disable_reflector(agent, 0);
            StatusModule::change_status_request_from_script(
                agent.module_accessor,
                *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END,
                true,
            );
        }
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        disable_reflector(agent, 0);
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_speciallw", ganon_speciallw, Priority::Default)
        .game_acmd("game_specialairlw", ganon_specialairlw, Priority::Default)
        .install();
}
