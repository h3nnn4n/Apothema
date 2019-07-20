use num::FromPrimitive;
use std::collections::HashMap;
use std::time::Instant;

use crate::cube::{reduntant_move, Cube, Move};
use super::*;

pub fn build_prunning_tables() -> PrunningTables {
    let mut prunning_tables: PrunningTables = PrunningTables::new();
    let mut cube = Cube::new();
    let moves = (0..18).into_iter().map(|key| Move::from_u32(key).unwrap());

    prunning_tables.state.queue.push_back((cube.cube_to_tuple(), Move::NOP, 0));
    prunning_tables.stats.start_timer();

    println!("Started building prunning table");

    loop {
        prunning_tables.stats_tick();

        if prunning_tables.state.queue.len() == 0 {
            panic!("Unsovable cube!");
        } else {
            let (cube_i, last_move, depth) = prunning_tables.state.queue.pop_front().unwrap();

            if depth > prunning_tables.state.max_depth {
                prunning_tables.state.max_depth = depth;

                prunning_tables.print_status();

                if prunning_tables.state.max_depth > 5 {
                    break;
                }
            }

            for mov in moves.clone() {
                if reduntant_move(mov, last_move) {
                    continue;
                }

                cube.cube_from_tuple(cube_i);
                cube.do_move(mov);

                let cube_tuple = cube.cube_to_tuple();

                if !prunning_tables.state.visited.contains(&cube_tuple) {
                    prunning_tables.state.queue.push_back((cube_tuple, mov, depth + 1));
                    prunning_tables.state.visited.insert(cube_tuple);
                    update_prunning_table(&mut prunning_tables, &cube, depth);
                }
            }
        }
    }

    let t_current = Instant::now();
    let t_diff = t_current.duration_since(prunning_tables.stats.t_start);
    let time = (t_diff.as_secs() as f64) + (t_diff.subsec_millis() as f64 / 1000.0);
    let moves_per_sec = (prunning_tables.state.visited.len() as f64) / time;

    println!();
    println!(
        "Finished building prunning table\ntime_elapsed {:4}.{:03}    visited {} nodes    moves_per_sec: {:8.0}",
        t_diff.as_secs(),
        t_diff.subsec_millis(),
        prunning_tables.state.visited.len(),
        moves_per_sec
    );

    prunning_tables
}

fn update_prunning_table(table: &mut PrunningTables, cube: &Cube, depth: u64) {
    update_edge_orientation(table, cube, depth);
    update_corner_orientation(table, cube, depth);

    update_edge_permutation(table, cube, depth);
    update_corner_permutation(table, cube, depth);
}

pub fn update_edge_orientation(table: &mut PrunningTables,cube: &Cube, depth: u64) {
    let edge_orientation = cube.edge_orientation_as_u64();
    let key = (edge_orientation, 0);
    let value = depth;

    table_update(&mut table.edge_orientation, key, value);
}

pub fn update_corner_orientation(table: &mut PrunningTables,cube: &Cube, depth: u64) {
    let corner_orientation = cube.corner_orientation_as_u64();
    let key = (corner_orientation, 0);
    let value = depth;

    table_update(&mut table.corner_orientation, key, value);
}

pub fn update_edge_permutation(table: &mut PrunningTables,cube: &Cube, depth: u64) {
    let edge_permutation = cube.edge_permutation_as_u64();
    let key = (edge_permutation, 0);
    let value = depth;

    table_update(&mut table.edge_permutation, key, value);
}

pub fn update_corner_permutation(table: &mut PrunningTables,cube: &Cube, depth: u64) {
    let corner_permutation = cube.corner_permutation_as_u64();
    let key = (corner_permutation, 0);
    let value = depth;

    table_update(&mut table.corner_permutation, key, value);
}

fn table_update(table: &mut HashMap<(u64, u64), u64>, key: (u64, u64), value: u64) {
    if table.contains_key(&key) {
        let table_depth = table[&key];

        if value < table_depth {
            table.insert(key, value);
        }
    } else {
        table.insert(key, value);
    }
}
