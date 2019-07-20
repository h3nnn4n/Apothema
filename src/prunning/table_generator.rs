use num::FromPrimitive;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::cube::{reduntant_move, Cube, Move};
use super::*;

pub fn build_edge_orientation_prunning_table() -> PrunningTables {
    let mut queue: VecDeque<((u64, u64), Move, u64)> = VecDeque::new();
    let mut prunning_tables: PrunningTables = PrunningTables::new();
    let mut visited: HashSet<(u64, u64)> = HashSet::new();

    let mut cube = Cube::new();
    let mut max_depth = 0;

    let moves = (0..18).into_iter().map(|key| Move::from_u32(key).unwrap());

    queue.push_back((cube.cube_to_tuple(), Move::NOP, 0));

    println!("Started building prunning table");

    let t_start = Instant::now();
    let mut t_timer = t_start;
    let t_delta = Duration::from_millis(1000);

    loop {
        let t_current = Instant::now();
        let t_diff = t_current.duration_since(t_timer);

        if t_diff > t_delta {
            t_timer = Instant::now();
            let time_elapsed = t_current.duration_since(t_start);
            print_status(&queue, &visited, time_elapsed, max_depth);
        }
        if queue.len() == 0 {
            panic!("Unsovable cube!");
        } else {
            let (cube_i, last_move, depth) = queue.pop_front().unwrap();

            if depth > max_depth {
                max_depth = depth;

                let time_elapsed = t_current.duration_since(t_start);
                print_status(&queue, &visited, time_elapsed, max_depth);

                if max_depth > 4 {
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

                if !visited.contains(&cube_tuple) {
                    queue.push_back((cube_tuple, mov, depth + 1));
                    visited.insert(cube_tuple);
                    update_prunning_table(&mut prunning_tables, &cube, depth);
                }
            }
        }
    }

    let t_current = Instant::now();
    let t_diff = t_current.duration_since(t_start);
    let time = (t_diff.as_secs() as f64) + (t_diff.subsec_millis() as f64 / 1000.0);
    let moves_per_sec = (visited.len() as f64) / time;

    println!();
    println!(
        "Finished building prunning table\ntime_elapsed {:4}.{:03}    visited {} nodes    moves_per_sec: {:8.0}",
        t_diff.as_secs(),
        t_diff.subsec_millis(),
        visited.len(),
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

fn print_status(
    queue: &VecDeque<((u64, u64), Move, u64)>,
    visited: &HashSet<(u64, u64)>,
    t_diff: std::time::Duration,
    max_depth: u64,
) {
    let time = (t_diff.as_secs() as f64) + (t_diff.subsec_millis() as f64 / 1000.0);
    let moves_per_sec = (visited.len() as f64) / time;

    println!(
        "time_elapsed: {:4}.{:03}    depth: {:2}    nodes_visited: {:12}    queue_size: {:12}    moves_per_sec: {:8.0}",
        t_diff.as_secs(),
        t_diff.subsec_millis(),
        max_depth,
        visited.len(),
        queue.len(),
        moves_per_sec
    );
}
