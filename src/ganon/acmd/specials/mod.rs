mod special_high;
mod special_low;
mod special_neutral;
mod special_side_backhand;

pub fn install() {
    special_neutral::install();
    special_low::install();
    special_side_backhand::install();
    special_high::install();
}
