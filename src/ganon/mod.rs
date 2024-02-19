mod acmd;
mod float;
mod frame;
mod omni_teleport;
mod utils;
mod warlock_punch;

pub fn install() {
    acmd::install();
    frame::install();
}
