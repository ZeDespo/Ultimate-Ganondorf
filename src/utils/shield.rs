//! Shield generic helpers.
use crate::imports::*;

/// Activates a reflector for a defined hitbox. This hitbox will always
/// reflect projectiles back at the oppoenent.
pub unsafe extern "C" fn activate_reflector(
    fighter: &mut L2CAgentBase,
    id: i32,
    bone: Hash40,
    size: f32,
    x: f32,
    y: f32,
    z: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    damage_multiplier: f32,
    speed_multiplier: f32,
    max_damage: i32,
    life_multiplier: f32,
) {
    shield!(
        fighter,
        *MA_MSC_CMD_REFLECTOR,
        *COLLISION_KIND_REFLECTOR,
        id,
        bone,
        size,
        x,
        y,
        z,
        x2,
        y2,
        z2,
        damage_multiplier,
        speed_multiplier,
        max_damage,
        false, // Unknown property
        life_multiplier,
        *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT
    );
}

/// Turn off a reflector hitbox.
pub unsafe extern "C" fn disable_reflector(fighter: &mut L2CAgentBase, id: i32) {
    shield!(
        fighter,
        *MA_MSC_CMD_SHIELD_OFF,
        *COLLISION_KIND_REFLECTOR,
        id,
        *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT
    );
}
