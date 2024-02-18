#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]

mod ganon;
mod utils;

#[skyline::main(name = "ultimate_ganondorf")]
pub fn main() {
    ganon::install()
}
