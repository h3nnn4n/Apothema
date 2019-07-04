#![feature(plugin)]
#![feature(proc_macro_hygiene)]

extern crate bincode;
extern crate num;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate test_case_derive;
#[macro_use]
extern crate enum_primitive;
extern crate phf;

mod cube;
mod search;

use cube::*;

fn main() {
    let mut cube = Cube::new();
    let random_moves = get_random_move_sequence(5);

    println!("{:?}", random_moves);

    cube.do_move_sequence(&random_moves);

    let seq = search::bfs_solver(&cube);

    println!("{:?}", seq);

    println!("Apothema!");
}
