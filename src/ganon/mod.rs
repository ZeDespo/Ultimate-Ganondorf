mod acmd;
mod float_utils;
mod frame;
mod warlock_punch_on_taunt;

pub fn install() {
    acmd::install();
    frame::install();
}
