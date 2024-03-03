use smash::lib::lua_const::*;

/// Is Status Kind an attack Status Kind?
/// 0x27 - 0x36

pub fn is_attack_status_kind(status_kind: &i32) -> bool {
    status_kind >= &FIGHTER_STATUS_KIND_ATTACK && status_kind <= &FIGHTER_STATUS_KIND_ATTACK_AIR
}
