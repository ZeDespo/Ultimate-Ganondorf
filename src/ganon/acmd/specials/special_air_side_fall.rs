//! This function is for the dive hitbox for piledrive
use crate::{
    ganon::utils::{
        GANON_DOWN_SPECIAL_AIR_CONTINUE_FLAG, GANON_DOWN_SPECIAL_AIR_COUNTDOWN_FLOAT,
        GANON_DOWN_SPECIAL_AIR_MULTIPLIER_FLAG,
    },
    imports::*,
};

unsafe extern "C" fn use_weak_hitbox(agent: &mut L2CAgentBase) -> bool {
    if !WorkModule::is_flag(agent.module_accessor, GANON_DOWN_SPECIAL_AIR_CONTINUE_FLAG) {
        let countdown = WorkModule::get_int(
            agent.module_accessor,
            GANON_DOWN_SPECIAL_AIR_COUNTDOWN_FLOAT,
        );
        return countdown >= 0 && countdown <= 10;
    }
    false
}

unsafe extern "C" fn ganon_specialairsfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0, -5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    let weak_hitbox = use_weak_hitbox(agent);
    let damage: f32;
    let kbg: i32;
    let bkb: i32;
    if !weak_hitbox {
        damage = 14.0;
        kbg = 113;
        bkb = 60;
    } else {
        damage = 10.0;
        kbg = 86;
        bkb = 50;
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(
            agent.module_accessor,
            *FIGHTER_GANON_EXPLOSION_FALL_SETTING_FALL,
            *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING,
        );
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("handl"),
            damage,
            80,
            kbg,
            0,
            bkb,
            3.5,
            0.0,
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
            *COLLISION_SITUATION_MASK_A,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_purple"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_HEAVY,
            *ATTACK_REGION_PUNCH,
        );
        if WorkModule::is_flag(agent.module_accessor, GANON_DOWN_SPECIAL_AIR_CONTINUE_FLAG) {
            macros::ATTACK(
                agent,
                1,
                0,
                Hash40::new("handl"),
                8.0,
                80,
                100,
                0,
                50,
                3.5,
                0.0,
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
                *COLLISION_SITUATION_MASK_G,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                Hash40::new("collision_attr_paralyze"),
                *ATTACK_SOUND_LEVEL_L,
                *COLLISION_SOUND_ATTR_PUNCH,
                *ATTACK_REGION_PUNCH,
            );
        } else {
            AttackModule::clear(agent.module_accessor, 1, false.into());
            macros::ATTACK(
                agent,
                2,
                0,
                Hash40::new("handl"),
                8.0,
                80,
                100,
                0,
                50,
                3.6,
                0.0,
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
                1,
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
                *ATTACK_REGION_PUNCH,
            );
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        if !weak_hitbox {
            WorkModule::on_flag(
                agent.module_accessor,
                GANON_DOWN_SPECIAL_AIR_MULTIPLIER_FLAG,
            );
        }
    }
}

unsafe extern "C" fn effect_specialairsfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("ganon_engokua_catch"),
            Hash40::new("havel"),
            -1,
            0,
            0.5,
            0,
            0,
            0,
            1,
            true,
        );
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd(
            "game_specialairsfall",
            ganon_specialairsfall,
            Priority::Default,
        )
        .game_acmd(
            "effect_specialairsfall",
            effect_specialairsfall,
            Priority::Default,
        )
        .install();
}
