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
    let random_moves = get_random_move_sequence(15);

    cube.do_move_sequence(&random_moves);

    let cube_tuple = cube.cube_to_tuple();

    // let serialized = serde_json::to_string(&cube_tuple).unwrap();
    let serialized = bincode::serialize(&cube_tuple).unwrap();

    // println!("serialized = {}", serialized);

    let deserialized: (u64, u64) = bincode::deserialize(&serialized).unwrap();
    // let deserialized: (u64, u64) = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);

    let mut new_cube = Cube::new();
    new_cube.cube_from_tuple(deserialized);

    let reverse_sequence = reverse_move_sequence(random_moves.clone());
    new_cube.do_move_sequence(&reverse_sequence);

    println!("{}", new_cube.is_solved());
    println!("{}", new_cube.is_valid());

    println!("Apothema!");
}
