mod aerials;
mod ground;
mod specials;

pub fn install() {
    ground::install();
    aerials::install();
    specials::install();
}
