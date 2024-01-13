mod acmd;
mod float_utils;
mod frame;

pub fn install() {
    acmd::install();
    frame::install();
}
