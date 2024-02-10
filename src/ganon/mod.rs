mod acmd;
mod float;
mod frame;
mod utils;
mod warlock_punch;

pub fn install() {
    acmd::install();
    frame::install();
}
