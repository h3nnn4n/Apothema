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
extern crate clap;
extern crate phf;
extern crate ron;

mod cube;
mod prunning;
mod search;

use clap::{App, Arg};
use cube::*;

fn main() {
    let matches = App::new("Apothema")
        .version("0.1")
        .author("Renan S Silva, aka h3nnn4n <uber.renan@gmail.com>")
        .about("Does awesome rubiky cube things")
        .arg(
            Arg::with_name("BUILD_PRUNNING")
                .short("-p")
                .long("--build-prunning-tables")
                .help("Builds the prunning tables"),
        )
        .get_matches();

    let build_prunning = matches.is_present("BUILD_PRUNNING");

    if build_prunning {
        let table = prunning::build_edge_orientation_prunning_table();
        prunning::store_table(table);
    } else {
        demo();
    }
}

fn demo() {
    let mut cube = Cube::new();
    let random_moves = get_random_move_sequence(5);

    println!("{:?}", random_moves);

    cube.do_move_sequence(&random_moves);

    let seq = search::bfs_solver(&cube);

    println!("{:?}", seq);

    println!("Apothema!");
}
