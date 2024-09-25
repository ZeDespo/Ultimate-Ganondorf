use crate::imports::*;
use crate::ganon::utils::*;

unsafe extern "C" fn attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi3();

    fighter.sub_shift_status_main(L2CValue::Ptr(attackhi3_main_loop as *const () as _))
}

unsafe extern "C" fn attackhi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);

    if frame >= 26.0
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }

    fighter.status_AttackHi3_Main();

    0.into()
}

pub fn install() {
    Agent::new("ganon")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI3, attackhi3_main)
        .install();
}
