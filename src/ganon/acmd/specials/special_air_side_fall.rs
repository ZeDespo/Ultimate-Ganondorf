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
        return countdown > 0 && countdown <= 10;
    }
    false
}

unsafe extern "C" fn ganon_specialairsfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0, -5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    for i in 1..19 {
        frame(agent.lua_state_agent, i as f32);
        let damage: f32;
        let kbg: i32;
        let bkb: i32;
        let collision: Hash40;
        if !use_weak_hitbox(agent) {
            damage = 14.0;
            kbg = 113;
            bkb = 60;
            collision = Hash40::new("collision_attr_paralyze");
        } else {
            damage = 10.0;
            kbg = 86;
            bkb = 50;
            collision = Hash40::new("collision_attr_purple");
        }
        if macros::is_excute(agent) {
            if i == 2 {
                WorkModule::set_int(
                    agent.module_accessor,
                    *FIGHTER_GANON_EXPLOSION_FALL_SETTING_FALL,
                    *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_FALL_SETTING,
                );
            } else if i == 19 && !use_weak_hitbox(agent) {
                WorkModule::on_flag(
                    agent.module_accessor,
                    GANON_DOWN_SPECIAL_AIR_MULTIPLIER_FLAG,
                );
            }
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
            macros::ATTACK(
                agent,
                1,
                0,
                Hash40::new("handl"),
                8.0,
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
                *COLLISION_SITUATION_MASK_G,
                *COLLISION_CATEGORY_MASK_ALL,
                *COLLISION_PART_MASK_ALL,
                false,
                collision,
                *ATTACK_SOUND_LEVEL_L,
                *COLLISION_SOUND_ATTR_PUNCH,
                *ATTACK_REGION_PUNCH,
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
