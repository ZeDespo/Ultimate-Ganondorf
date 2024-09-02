mod attack_dash;
mod attack_down_smash;
mod attack_down_tilt;
mod attack_forward_smash;
mod attack_up_smash;
mod attack_up_tilt;

pub fn install() {
    attack_up_tilt::install();
    attack_up_smash::install();
    attack_dash::install();
    attack_forward_smash::install();
    attack_down_smash::install();
    attack_down_tilt::install();
}
