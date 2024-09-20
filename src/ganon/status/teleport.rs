use crate::ganon::utils::*;
use crate::imports::*;

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

unsafe extern "C" fn end_teleport(fighter: &mut L2CFighterCommon) {
    macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    WorkModule::set_int(
        fighter.module_accessor,
        TeleportStatus::Ready as i32,
        GANON_TELEPORT_WORK_INT,
    );
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
        boma: &mut BattleObjectModuleAccessor,
    ) -> Position2D {
        let mut x = calculate_base_teleport_distance(ControlModule::get_stick_x(boma));
        let mut y = calculate_base_teleport_distance(ControlModule::get_stick_y(boma));
        if y == 0.0 {
            x = add_teleport_distance(x);
        } else if x == 0.0 {
            y = add_teleport_distance(y);
        }
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            if y < 0.0 {
                y = 0.0
            }
        }
        Position2D { x: x, y: y }
    }

    unsafe extern "C" fn set_to_array(self: Self, entry_id: usize) {
        GS[entry_id].teleport_direction = self;
    }
}

unsafe extern "C" fn teleport_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, GANON_CAN_TELEPORT_FLAG) {
        StatusModule::change_status_request_from_script(
            boma,
            FIGHTER_GANON_STATUS_KIND_PRE_TELEPORT,
            false.into(),
        );
        return 0.into();
    } else {
        return 1.into();
    }
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
    let boma = &mut *fighter.module_accessor;
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
            if frame == TELEPORT_START_INTANGIBILITY_FRAME {
                macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
                VisibilityModule::set_whole(fighter.module_accessor, false);
                JostleModule::set_status(fighter.module_accessor, false);
            }
            if frame == TELEPORT_TRANSIT_FRAME {
                KineticModule::clear_speed_all(boma);
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
            GroundModule::set_correct(
                fighter.module_accessor,
                GroundCorrectKind(*GROUND_CORRECT_KIND_AIR),
            );
            WorkModule::set_int(
                boma,
                TeleportStatus::EndTransit as i32,
                GANON_TELEPORT_WORK_INT,
            );
        }
        TeleportStatus::EndTransit => {
            if boma.is_situation(*SITUATION_KIND_GROUND)
                && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR
            {
                GroundModule::correct(
                    fighter.module_accessor,
                    GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP),
                );
                KineticModule::change_kinetic(
                    fighter.module_accessor,
                    *FIGHTER_KINETIC_TYPE_GROUND_STOP,
                );
                end_teleport(fighter);
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 1.into();
            }
            WorkModule::set_int(boma, TeleportStatus::End as i32, GANON_TELEPORT_WORK_INT);
            KineticModule::clear_speed_all(boma);
            return 0.into();
        }
        TeleportStatus::End => {
            KineticModule::clear_speed_all(boma);
            if frame >= TELEPORT_FRAMES || MotionModule::is_end(boma) {
                end_teleport(fighter);
                if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                } else {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
            }
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn teleport_calculator_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    end_teleport(fighter);
    0.into()
}
