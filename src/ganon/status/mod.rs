mod backhand;
mod teleport;
mod down_special;

pub fn install() {
    teleport::install();
    backhand::install();
    down_special::install();
}
