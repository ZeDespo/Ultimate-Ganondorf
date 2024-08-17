//! New Down Special
//!
//! Ganondorf plunges into the ground, causing an explosion.
//!
use crate::imports::*;
use crate::ganon::utils::*;

const SITUATION_KIND: i32 = 0x16;

struct DarkRuptureStats {
    id: u64,
    damage: f32,
    angle: u64,
    kbg: i32,
    bkb: i32,
    size: f32,
}

impl DarkRuptureStats {
    fn new(frame: f32) -> Option<DarkRuptureStats> {
        match frame as i8 {
            1 | 2 => Some(DarkRuptureStats {
                id: 0,
                damage: 20.0,
                angle: 361,
                kbg: 100,
                bkb: 40,
                size: 5.5,
            }),
            3..=6 => Some(DarkRuptureStats {
                id: 1,
                damage: 12.0,
                angle: 73,
                kbg: 80,
                bkb: 20,
                size: 10.0,
            }),
            7..=13 => Some(DarkRuptureStats {
                id: 2,
                damage: 8.0,
                angle: 45,
                kbg: 60,
                bkb: 20,
                size: 18.0,
            }),
            _ => None,
        }
    }
}

/// There is no ACMD entry that I can find for Ganondorf's end status for special air,
/// so imma do this myself.
unsafe extern "C" fn dark_rupture_hitboxes(fighter: &mut L2CFighterCommon) {
    let drs = DarkRuptureStats::new(MotionModule::frame(fighter.module_accessor));
    if let Some(dr) = drs {
        macros::ATTACK(
            fighter,
            dr.id,
            0,
            Hash40::new("handl"),
            dr.damage,
            dr.angle,
            dr.kbg,
            0,
            dr.bkb,
            dr.size,
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
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_MAGIC,
        );
    } else {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn fun_7100010b20(agent: &mut L2CFighterCommon) {
    agent.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(
        agent.module_accessor,
        GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND),
    );
}

unsafe extern "C" fn fun_7100006ef0(
    agent: &mut L2CFighterCommon,
    param_2: L2CValue,
    param_3: L2CValue,
) {
    let al_stack80: u64 = 0x32e468d950;
    agent.clear_lua_stack();
    agent.push_lua_stack(&mut L2CValue::new_int(al_stack80));
    agent.push_lua_stack(&mut L2CValue::new_int(param_2.get_u64()));
    agent.push_lua_stack(&mut L2CValue::new_int(param_3.get_u64()));
    notify_event_msc_cmd(agent.lua_state_agent);
    agent.pop_lua_stack(1);
}

unsafe extern "C" fn ganon_specialairsend_init(agent: &mut L2CFighterCommon) -> L2CValue {
    if in_dive(agent.module_accessor) {
        WorkModule::on_flag(agent.module_accessor, GANON_DARK_RUPTURE_ACTIVE);
        AttackModule::clear_all(agent.module_accessor);
    }
    fun_7100006ef0(
        agent,
        hash40("catched_ganon").into(),
        hash40("catched_air_end_ganon").into(),
    );
    0.into()
}

unsafe extern "C" fn ganon_specialairsend_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let h_var3 = Hash40::new_raw(0xd2b3a620b); // special_air_s
    let f_var4 = 0.0;
    let f_var5 = 1.0;
    let b_var1 = false;
    MotionModule::change_motion(
        agent.module_accessor,
        h_var3,
        f_var4,
        f_var5,
        b_var1,
        0.0,
        false,
        false,
    );
    fun_7100010b20(agent);
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    agent.sub_shift_status_main(L2CValue::Ptr(
        ganon_specialarsend_main_loop as *const () as _,
    ))
}

unsafe extern "C" fn ganon_specialarsend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !CancelModule::is_enable_cancel(boma) {
        let curr_situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        if curr_situation_kind != *SITUATION_KIND_GROUND {
            if curr_situation_kind != *SITUATION_KIND_AIR {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        } else {
            if !MotionModule::is_end(boma) {
                if WorkModule::is_flag(fighter.module_accessor, GANON_DARK_RUPTURE_ACTIVE) {
                    dark_rupture_hitboxes(fighter);
                }
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    } else {
        if fighter
            .sub_wait_ground_check_common(L2CValue::Bool(false))
            .get_bool()
            && fighter.sub_air_check_fall_common().get_bool()
        {
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn ganon_specialairsend_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, GANON_DOWN_SPECIAL_AIR);
    WorkModule::off_flag(fighter.module_accessor, GANON_DARK_RUPTURE_ACTIVE);
    CatchModule::catch_cut(fighter.module_accessor, false, false);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
        .status(
            Init,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
            ganon_specialairsend_init,
        )
        .status(
            Main,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
            ganon_specialairsend_main,
        )
        .status(
            Exit,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END,
            ganon_specialairsend_exit,
        )
        .install();
}
