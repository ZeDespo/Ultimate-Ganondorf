use skyline_smash::app::{ArticleOperationTarget, AttackDirectionAxis};

use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};
unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("havel"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.4,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.4,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
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
        wait(agent.lua_state_agent, 3.0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
    frame(agent.lua_state_agent, 16.0);
    for _ in 0..7 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
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
                Hash40::new("sys_thunder"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("throw"),
                0,
                0,
                -1,
                0,
                5,
                0,
                0.5,
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
                Hash40::new("sys_damage_elec"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("havel"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
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
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_damage_elec"),
            Hash40::new("throw"),
            0,
            0,
            0,
            0,
            0,
            0,
            0.85,
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
            Hash40::new("sys_damage_elec"),
            Hash40::new("throw"),
            0,
            0,
            0,
            2,
            3,
            5,
            0.95,
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
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
}

unsafe extern "C" fn effect_attacks4charge(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.3);
            macros::EFFECT(
                agent,
                Hash40::new("sys_damage_elec"),
                Hash40::new("throw"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.3);
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.3);
            macros::EFFECT(
                agent,
                Hash40::new("sys_thunder"),
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 0.3);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

unsafe extern "C" fn expression_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ArticleModule::set_visibility_whole(
            agent.module_accessor,
            *FIGHTER_GANON_GENERATE_ARTICLE_SWORD,
            false,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
    frame(agent.lua_state_agent, 15.0);
    execute(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ArticleModule::set_visibility_whole(
            agent.module_accessor,
            *FIGHTER_GANON_GENERATE_ARTICLE_SWORD,
            false,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("haver"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl_l"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_impact"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
}

pub fn install() {
    Agent::new("ganon")
        .effect_acmd("effect_attacks4", effect_attacks4, Priority::Default)
        .effect_acmd(
            "effect_attacks4charge",
            effect_attacks4charge,
            Priority::Default,
        )
        .expression_acmd(
            "expression_attacks4",
            expression_attacks4,
            Priority::Default,
        )
        .install();
}
