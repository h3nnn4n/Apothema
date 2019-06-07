#![feature(plugin)]
#![feature(proc_macro_hygiene)]

extern crate num;
#[macro_use]
extern crate enum_primitive;
extern crate phf;

mod cube;

fn main() {
    println!("Apothema!");
}
