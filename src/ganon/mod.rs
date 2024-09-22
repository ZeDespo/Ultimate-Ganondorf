mod acmd;
mod down_tilt_followup;
mod down_air_stall;
mod float;
mod float_check;
mod frame;
mod status;
mod teleport_check;
mod utils;
mod warlock_punch;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}
