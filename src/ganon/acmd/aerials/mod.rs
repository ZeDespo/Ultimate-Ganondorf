mod attack_back_air;
mod attack_forward_air;
mod attack_low_air;
mod attack_neutral_air;
mod attack_up_air;

pub fn install() {
    attack_up_air::install();
    attack_neutral_air::install();
    attack_low_air::install();
    attack_forward_air::install();
    attack_back_air::install();
}
