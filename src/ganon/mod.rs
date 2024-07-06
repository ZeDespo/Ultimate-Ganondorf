mod acmd;
mod float;
mod float_check;
mod frame;
mod new_down_special;
mod status;
mod teleport_check;
mod utils;
mod warlock_punch;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}
