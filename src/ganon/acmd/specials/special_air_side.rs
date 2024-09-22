//! This function is for the ground/landing hitbox for piledrive
use crate::{ganon::utils::GANON_DOWN_SPECIAL_AIR_DURATION_FLAG, imports::*};

unsafe extern "C" fn explosion_hitbox(agent: &mut L2CAgentBase, id: u64, damage: f32, angle: u64, size: f32, kbg: i32, bkb: i32) {
    let increase_power = WorkModule::is_flag(agent.module_accessor, GANON_DOWN_SPECIAL_AIR_DURATION_FLAG);
    let shield_damage = if id == 0 {4} else {1};
    let (mut d, mut s) = (damage, size);
    let (mut k, mut b) = ( kbg, bkb);
    if increase_power {
        d *= 1.5;
        s *= 1.5;
        k += 20;
        b += 20;
    }
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            id,
            0,
            Hash40::new("handl"),
            d,
            angle,
            k,
            0,
            b,
            s,
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
            shield_damage,
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
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_MAGIC,
        );
    }

}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    explosion_hitbox(agent, 0, 20.0, 361, 5.5, 100, 40);
    frame(agent.lua_state_agent, 3.0);
    explosion_hitbox(agent, 1, 12.0, 73, 10.0, 80, 30);
    frame(agent.lua_state_agent, 7.0);
    explosion_hitbox(agent, 2, 8.0, 45, 18.0, 70, 20);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialairs", game_specialairs, Priority::Default)
        .install();
}
