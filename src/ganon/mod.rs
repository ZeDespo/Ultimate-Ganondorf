mod acmd;
mod float;
mod frame;
mod new_down_special;
mod omni_teleport;
mod status;
mod utils;
mod warlock_punch;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}
