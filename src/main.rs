#![feature(plugin)]
#![feature(proc_macro_hygiene)]

extern crate num;
extern crate rand;
extern crate test_case_derive;
#[macro_use]
extern crate enum_primitive;
extern crate phf;

mod cube;

fn main() {
    println!("Apothema!");
}
