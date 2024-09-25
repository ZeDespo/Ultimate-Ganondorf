mod backhand;
mod teleport;
mod down_special;
mod up_tilt;

pub fn install() {
    teleport::install();
    backhand::install();
    down_special::install();
    up_tilt::install();
}
