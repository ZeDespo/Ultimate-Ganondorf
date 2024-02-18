mod attack_up_smash;
mod attack_up_tilt;

pub fn install() {
    attack_up_tilt::install();
    attack_up_smash::install();
}
