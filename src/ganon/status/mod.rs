mod backhand;
mod dark_rupture;
mod teleport;
mod down_special;

pub fn install() {
    dark_rupture::install();
    teleport::install();
    backhand::install();
    down_special::install();
}
