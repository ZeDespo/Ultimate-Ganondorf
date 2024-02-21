use super::utils::*;
use smash::{
    app::{lua_bind::*, sv_animcmd::*, *},
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*,
};
use smash_script::*;

fn is_special_n(motion_kind: u64) -> bool {
    motion_kind == hash40("special_n")
}

impl Position2D {
    unsafe extern "C" fn next_teleport_position(
        boma: *mut BattleObjectModuleAccessor,
    ) -> Position2D {
        Position2D { x: 40.0, y: 40.0 }
    }

    unsafe extern "C" fn set_to_work_module(self: &Self, boma: *mut BattleObjectModuleAccessor) {
        WorkModule::set_float(boma, self.x, GANON_TELEPORT_NEW_X_POS);
        WorkModule::set_float(boma, self.y, GANON_TELEPORT_NEW_Y_POS);
    }
}

impl TeleportStatus {
    fn suspend_kinetic_energy(self: &Self) -> bool {
        return match self {
            TeleportStatus::PreTransit | TeleportStatus::Transit | TeleportStatus::End => true,
            _ => false,
        };
    }
}

pub unsafe extern "C" fn ganon_teleport_handler(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let pre_ts_int = WorkModule::get_int(boma, GANON_TELEPORT_WORK_INT);
    println!("Teleport Status {:#?}", pre_ts_int);
    let ts = TeleportStatus::from_int(pre_ts_int);
    let curr_situation_kind = StatusModule::situation_kind(boma);
    if !StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_N
        && !curr_situation_kind == SITUATION_KIND_GROUND
    {
        return;
    }
    match ts {
        TeleportStatus::Start => {
            Position2D::next_teleport_position(boma).set_to_work_module(boma);
        }
        TeleportStatus::PreTransit => {
            PostureModule::add_pos_2d(
                boma,
                &Vector2f {
                    x: WorkModule::get_float(boma, GANON_TELEPORT_NEW_X_POS),
                    y: WorkModule::get_float(boma, GANON_TELEPORT_NEW_Y_POS),
                },
            );
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
            WorkModule::set_int(
                boma,
                TeleportStatus::Transit as i32,
                GANON_TELEPORT_WORK_INT,
            );
        }
        _ => {}
    }
    if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR
        && ts.suspend_kinetic_energy()
        && !WorkModule::is_flag(boma, GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG)
    {
        WorkModule::set_flag(boma, true, GANON_TELEPORT_INTO_FLOAT_INIT_FLAG);
        WorkModule::set_int(
            boma,
            TeleportStatus::NotApplicable as i32,
            GANON_TELEPORT_WORK_INT,
        );
    }
}
