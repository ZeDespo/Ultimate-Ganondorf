#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros, unused_imports)]

mod imports;
mod ganon;
mod utils;

#[skyline::main(name = "ultimate_ganondorf")]
pub fn main() {
    ganon::install()
}
