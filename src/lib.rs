#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros, unused_imports)]

mod ganon;
mod imports;
mod utils;

#[skyline::main(name = "ultimate_ganondorf")]
pub fn main() {
    ganon::install();
    smashline::clone_weapon("mario", "fireball", "ganon", "backair_projectile", true);
}
