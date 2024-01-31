mod special_low;
mod special_neutral;

pub fn install() {
    special_neutral::install();
    special_low::install();
}
