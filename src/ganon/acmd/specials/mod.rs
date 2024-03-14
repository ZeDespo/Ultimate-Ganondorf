mod special_low;
mod special_neutral;
mod special_side;

pub fn install() {
    special_neutral::install();
    special_low::install();
    special_side::install();
}
