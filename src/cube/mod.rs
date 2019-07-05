#![allow(dead_code)]
mod benchmarks;
mod cube_corner_encoding;
mod cube_display;
mod cube_edge_encoding;
mod cube_encoding;
mod cube_impl;
mod cube_sanity_check;
mod cube_types;
mod move_sequence_impl;
mod move_sequence_types;
mod move_sequence_utils;
mod move_table;
mod move_utils;
mod print_move_table;

pub use benchmarks::{fixed_move_per_sec, random_move_per_sec};
pub use cube_corner_encoding::*;
pub use cube_display::*;
pub use cube_edge_encoding::*;
pub use cube_encoding::*;
pub use cube_impl::*;
pub use cube_sanity_check::*;
pub use cube_types::*;
pub use move_sequence_impl::*;
pub use move_sequence_types::*;
pub use move_sequence_utils::*;
pub use move_table::*;
pub use move_utils::*;
pub use print_move_table::{print_corner_move_table, print_edge_move_table};

#[cfg(test)]
mod cube_encoding_tests;
#[cfg(test)]
mod cube_tests;
