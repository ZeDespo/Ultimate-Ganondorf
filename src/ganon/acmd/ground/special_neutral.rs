use super::*;
// use crate::util::*;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::app::utility::get_kind;
use smash::app::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::{L2CAgent, L2CValue};
use smash::lua2cpp::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash_script::*;
use smashline::*;
use std::mem;

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_specialn", ganon_teleport)
        .effect_acmd("effect_specialn", ganon_teleport_eff)
        .sound_acmd("sound_specialn", ganon_teleport_snd)
        .expression_acmd("expression_specialn", ganon_teleport_expr)
        .install();
}

unsafe extern "C" fn ganon_teleport(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        let y_ctrl = ControlModule::get_stick_y(fighter.module_accessor);
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        VisibilityModule::set_whole(fighter.module_accessor, false);
        JostleModule::set_status(fighter.module_accessor, false);
        macros::SET_SPEED_EX(fighter, 7.2, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        VisibilityModule::set_whole(fighter.module_accessor, true);
        JostleModule::set_status(fighter.module_accessor, true);
        macros::SET_SPEED_EX(fighter, 0.0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            /*ID*/ 0,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("hip"),
            /*Damage*/ 24.0,
            /*Angle*/ 361,
            /*KBG*/ 100,
            /*FKB*/ 150,
            /*BKB*/ 0,
            /*Size*/ 12.0,
            /*X*/ 0.0,
            /*Y*/ 0.0,
            /*Z*/ 0.0,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 1.0,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_POS,
            /*SetWeight*/ true,
            /*ShieldDamage*/ -10,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ false,
            /*Absorbable*/ false,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ true,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_purple"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L,
            /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE,
            /*Type*/ *ATTACK_REGION_PUNCH,
        );
        macros::ATTACK(
            fighter,
            /*ID*/ 1,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("hip"),
            /*Damage*/ 24.0,
            /*Angle*/ 361,
            /*KBG*/ 100,
            /*FKB*/ 120,
            /*BKB*/ 0,
            /*Size*/ 8.0,
            /*X*/ 0.0,
            /*Y*/ 0.0,
            /*Z*/ 0.0,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 1.0,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_POS,
            /*SetWeight*/ true,
            /*ShieldDamage*/ -10,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ false,
            /*Absorbable*/ false,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ true,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_purple"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L,
            /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE,
            /*Type*/ *ATTACK_REGION_PUNCH,
        );
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn ganon_teleport_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
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
    let lua_state = fighter.lua_state_agent;
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
    let lua_state = fighter.lua_state_agent;
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attack_critical"), 0);
    }
}

// #[acmd_script(
//     agent = "ganon",
//     script =  "game_specialairn",
//     category = ACMD_GAME,
// 	low_priority)]
// unsafe fn ganon_float(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
// }
//
// #[acmd_script(
//     agent = "ganon",
//     script =  "sound_specialairn",
//     category = ACMD_SOUND,
// 	low_priority)]
// unsafe fn ganon_floats(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
// }
// #[acmd_script(
//     agent = "ganon",
//     script =  "effect_specialairn",
//     category = ACMD_EFFECT,
// 	low_priority)]
// unsafe fn ganon_floate(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
// }
// #[acmd_script(
//     agent = "ganon",
//     script =  "expression_specialairn",
//     category = ACMD_EXPRESSION,
// 	low_priority)]
// unsafe fn ganon_float_expr(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
// }
