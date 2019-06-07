#![feature(plugin)]
#![feature(proc_macro_hygiene)]

extern crate num;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate phf;
extern crate phf_codegen;

mod cube;

fn main() {
    println!("Apothema!");
}
