use crate::ganon::utils::in_dive;
use crate::ganon::utils::InitValues;
use crate::ganon::utils::Position2D;
use crate::ganon::utils::TeleportStatus;
use crate::ganon::utils::FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT;
use crate::ganon::utils::GANON_DARK_RUPTURE_ACTIVE;
use crate::ganon::utils::GANON_FLOAT_INTO_DIVE;
use crate::ganon::utils::GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG;
use crate::ganon::utils::GANON_TELEPORT_INTO_FLOAT_INIT_FLAG;
use crate::ganon::utils::GANON_TELEPORT_WORK_INT;
use crate::ganon::utils::GS;
use skyline_smash::app::BattleObjectModuleAccessor;
use skyline_smash::app::GroundCorrectKind;
use skyline_smash::phx::Vector2f;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::app::sv_battle_object::notify_event_msc_cmd;
use smash::lib::lua_const::*;
use smash_script::{damage, lua_args, macros, slope};
use {
    smash::{hash40, lua2cpp::*},
    smashline::*,
};
const MIN_TELEPORT_STEP: f32 = 20.0;
const MID_TELEPORT_STEP: f32 = 40.0;
pub fn install() {
    Agent::new("ganon")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, teleport_init)
        .status(
            Init,
            FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
            teleport_calculator_init,
        )
        .status(
            Main,
            FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
            teleport_calculator_main,
        )
        .status(
            Exit,
            FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
            teleport_calculator_exit,
        )
        .install();
}

/// Distance is measured by mash ferocity and direction of the left analog stick
fn calculate_base_teleport_distance(stick: f32) -> f32 {
    let stick_abs = stick.abs();
    let mut t_step = 0.0;
    if stick_abs > 0.2 && stick_abs <= 0.8 {
        t_step = MIN_TELEPORT_STEP;
    } else if stick_abs > 0.8 {
        t_step = MID_TELEPORT_STEP;
    }
    if stick < 0.0 {
        return -t_step;
    }
    t_step
}

/// Add teleport distance iff the
fn add_teleport_distance(direction: f32) -> f32 {
    if direction < 0.0 {
        return direction - 20.0;
    } else if direction > 0.0 {
        return direction + 20.0;
    }
    direction
}

unsafe extern "C" fn teleport_fx(fighter: &mut L2CFighterCommon) {
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
    macros::LAST_EFFECT_SET_RATE(fighter, 1.875); // 2.5 == 30 frames
    macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_l01"));
}

impl Position2D {
    // If on the ground, there are five different positions to choose from:
    // Up Right vertex:   x: 0.9166667, y: 0.5
    // Up Left vertex     x: -0.9166667, y: 0.5
    // Down Right vertex: x: 0.9166667, y: -0.5
    // Down Left vertex:  x: -0.9166667, y: -0.5
    unsafe extern "C" fn next_teleport_position(
        boma: *mut BattleObjectModuleAccessor,
    ) -> Position2D {
        let mut x = calculate_base_teleport_distance(ControlModule::get_stick_x(boma));
        let mut y = calculate_base_teleport_distance(ControlModule::get_stick_y(boma));
        if y == 0.0 {
            x = add_teleport_distance(x);
        } else if x == 0.0 {
            y = add_teleport_distance(y);
        }
        if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND && y < 0.0 {
            y = 0.0;
        }
        Position2D { x: x, y: y + 0.1 }
    }

    unsafe extern "C" fn set_to_array(self: Self, entry_id: usize) {
        GS[entry_id].teleport_direction = self;
    }
}

unsafe extern "C" fn teleport_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::change_status_request_from_script(
        fighter.module_accessor,
        FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
        false.into(),
    );
    0.into()
}

unsafe extern "C" fn teleport_calculator_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(
        fighter.module_accessor,
        TeleportStatus::PreTransit as i32,
        GANON_TELEPORT_WORK_INT,
    );
    0.into()
}

unsafe extern "C" fn teleport_calculator_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(
        teleport_calculator_main_loop as *const () as _,
    ))
}

unsafe extern "C" fn teleport_calculator_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let frame = MotionModule::frame(boma);
    let ts = TeleportStatus::from_int(WorkModule::get_int(boma, GANON_TELEPORT_WORK_INT));
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    println!("Teleport Status: {}", ts);
    match ts {
        TeleportStatus::Ready => WorkModule::set_int(
            boma,
            TeleportStatus::PreTransit as i32,
            GANON_TELEPORT_WORK_INT,
        ),
        TeleportStatus::PreTransit => {
            Position2D::next_teleport_position(boma).set_to_array(entry_id);
            if frame == 16.0 {
                WorkModule::set_int(
                    boma,
                    TeleportStatus::Transit as i32,
                    GANON_TELEPORT_WORK_INT,
                );
            }
        }
        TeleportStatus::Transit => {
            let teleport_position = GS[entry_id].teleport_direction;
            PostureModule::add_pos_2d(
                boma,
                &Vector2f {
                    x: teleport_position.x,
                    y: teleport_position.y,
                },
            );
            macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
            VisibilityModule::set_whole(fighter.module_accessor, false);
            JostleModule::set_status(fighter.module_accessor, false);
            GroundModule::set_correct(
                fighter.module_accessor,
                GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
            );
            teleport_fx(fighter);
            WorkModule::set_int(boma, TeleportStatus::End as i32, GANON_TELEPORT_WORK_INT);
            if !WorkModule::is_flag(
                fighter.module_accessor,
                GANON_TELEPORT_INTO_FLOAT_HANDLE_FLAG,
            ) {
                WorkModule::set_flag(boma, true, GANON_TELEPORT_INTO_FLOAT_INIT_FLAG);
                WorkModule::set_int(boma, TeleportStatus::Ready as i32, GANON_TELEPORT_WORK_INT);
            }
            return 0.into();
        }
        TeleportStatus::End => {
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn teleport_calculator_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(
        fighter.module_accessor,
        TeleportStatus::Ready as i32,
        GANON_TELEPORT_WORK_INT,
    );
    0.into()
}
