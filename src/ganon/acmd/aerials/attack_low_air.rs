//! D-Air can be cancelled on frame 30 instead of frame 32.
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::Hash40;
use smash_script::*;
use smashline::*;

const DAIR_LENGTH: f32 = 25.0;
const DAIR_FRAME: f32 = 22.0;

unsafe extern "C" fn ganon_attackairlw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.65);
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
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        // macros::ATTACK(
        //     agent,
        //     0,
        //     0,
        //     Hash40::new("top"),
        //     19.0,
        //     58,
        //     61,
        //     0,
        //     87,
        //     4.3,
        //     0.0,
        //     2.0,
        //     1.0,
        //     None,
        //     None,
        //     None,
        //     1.1,
        //     1.0,
        //     *ATTACK_SETOFF_KIND_ON,
        //     *ATTACK_LR_CHECK_POS,
        //     false,
        //     9,
        //     0.0,
        //     0,
        //     false,
        //     false,
        //     false,
        //     false,
        //     true,
        //     *COLLISION_SITUATION_MASK_A,
        //     *COLLISION_CATEGORY_MASK_ALL,
        //     *COLLISION_PART_MASK_ALL,
        //     false,
        //     Hash40::new("collision_attr_normal"),
        //     *ATTACK_SOUND_LEVEL_M,
        //     *COLLISION_SOUND_ATTR_ELEC,
        //     *ATTACK_REGION_MAGIC,
        // );
        // macros::ATTACK(
        //     agent,
        //     1,
        //     0,
        //     Hash40::new("top"),
        //     19.0,
        //     72,
        //     56,
        //     0,
        //     97,
        //     4.3,
        //     0.0,
        //     2.0,
        //     1.0,
        //     None,
        //     None,
        //     None,
        //     1.3,
        //     1.0,
        //     *ATTACK_SETOFF_KIND_ON,
        //     *ATTACK_LR_CHECK_POS,
        //     false,
        //     9,
        //     0.0,
        //     0,
        //     false,
        //     false,
        //     false,
        //     false,
        //     true,
        //     *COLLISION_SITUATION_MASK_G,
        //     *COLLISION_CATEGORY_MASK_ALL,
        //     *COLLISION_PART_MASK_ALL,
        //     false,
        //     Hash40::new("collision_attr_normal"),
        //     *ATTACK_SOUND_LEVEL_M,
        //     *COLLISION_SOUND_ATTR_ELEC,
        //     *ATTACK_REGION_MAGIC,
        // );
        // macros::ATTACK(
        //     agent,
        //     4,
        //     0,
        //     Hash40::new("top"),
        //     19.0,
        //     58,
        //     61,
        //     0,
        //     87,
        //     4.0,
        //     0.0,
        //     -3.0,
        //     1.0,
        //     None,
        //     None,
        //     None,
        //     1.1,
        //     1.0,
        //     *ATTACK_SETOFF_KIND_ON,
        //     *ATTACK_LR_CHECK_POS,
        //     false,
        //     9,
        //     0.0,
        //     0,
        //     false,
        //     false,
        //     false,
        //     false,
        //     true,
        //     *COLLISION_SITUATION_MASK_A,
        //     *COLLISION_CATEGORY_MASK_ALL,
        //     *COLLISION_PART_MASK_ALL,
        //     false,
        //     Hash40::new("collision_attr_normal"),
        //     *ATTACK_SOUND_LEVEL_M,
        //     *COLLISION_SOUND_ATTR_ELEC,
        //     *ATTACK_REGION_MAGIC,
        // );
        // macros::ATTACK(
        //     agent,
        //     5,
        //     0,
        //     Hash40::new("top"),
        //     19.0,
        //     72,
        //     56,
        //     0,
        //     97,
        //     4.0,
        //     0.0,
        //     -3.0,
        //     1.0,
        //     None,
        //     None,
        //     None,
        //     1.3,
        //     1.0,
        //     *ATTACK_SETOFF_KIND_ON,
        //     *ATTACK_LR_CHECK_POS,
        //     false,
        //     9,
        //     0.0,
        //     0,
        //     false,
        //     false,
        //     false,
        //     false,
        //     true,
        //     *COLLISION_SITUATION_MASK_G,
        //     *COLLISION_CATEGORY_MASK_ALL,
        //     *COLLISION_PART_MASK_ALL,
        //     false,
        //     Hash40::new("collision_attr_normal"),
        //     *ATTACK_SOUND_LEVEL_M,
        //     *COLLISION_SOUND_ATTR_ELEC,
        //     *ATTACK_REGION_MAGIC,
        // );
        // macros::ATTACK(
        //     agent,
        //     2,
        //     0,
        //     Hash40::new("haver"),
        //     19.0,
        //     270,
        //     86,
        //     0,
        //     26,
        //     4.5,
        //     0.0,
        //     13.5,
        //     2.0,
        //     None,
        //     None,
        //     None,
        //     1.5,
        //     1.0,
        //     *ATTACK_SETOFF_KIND_ON,
        //     *ATTACK_LR_CHECK_POS,
        //     false,
        //     9,
        //     0.0,
        //     0,
        //     false,
        //     false,
        //     false,
        //     false,
        //     true,
        //     *COLLISION_SITUATION_MASK_A,
        //     *COLLISION_CATEGORY_MASK_ALL,
        //     *COLLISION_PART_MASK_ALL,
        //     false,
        //     Hash40::new("collision_attr_normal"),
        //     *ATTACK_SOUND_LEVEL_L,
        //     *COLLISION_SOUND_ATTR_ELEC,
        //     *ATTACK_REGION_MAGIC,
        // );
        macros::ATTACK(
            agent,
            3,
            0,
            Hash40::new("haver"),
            19.0,
            270,
            78,
            0,
            97,
            2.5,
            0.0,
            0.0,
            0.0,
            Some(0.0),
            Some(25.0),
            Some(0.0),
            1.5,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
            false,
            9,
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
    wait(agent.lua_state_agent, 3.0);
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
    frame(agent.lua_state_agent, DAIR_FRAME);
    if macros::is_excute(agent) {
        let mut i: f32 = 0.0;
        while i <= DAIR_LENGTH {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("ganon_attack_elec"),
                Hash40::new("haver"),
                0,
                i as i32,
                0,
                0,
                0,
                0,
                1.0,
                true,
            );
            i = i + 5.0;
        }
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
        .game_acmd("game_attackairlw", ganon_attackairlw)
        .effect_acmd("effect_attackairlw", effect_attackairlw)
        .sound_acmd("sound_attackairlw", sound_attackairlw)
        .install();
}
