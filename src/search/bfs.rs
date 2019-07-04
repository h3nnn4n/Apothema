use num::FromPrimitive;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::cube::{reduntant_move, Cube, Move};

pub fn bfs_solver(starting_cube: &Cube) -> Vec<Move> {
    let mut queue: VecDeque<((u64, u64), Move)> = VecDeque::new();
    let mut reverse_path: HashMap<(u64, u64), ((u64, u64), Move)> = HashMap::new();
    let mut visited: HashSet<(u64, u64)> = HashSet::new();

    let mut cube = Cube::new();

    let moves = (0..18).into_iter().map(|key| Move::from_u32(key).unwrap());

    queue.push_back((starting_cube.cube_to_tuple(), Move::NOP));

    loop {
        if queue.len() == 0 {
            panic!("Unsovable cube!");
        } else {
            let (cube_i, last_move) = queue.pop_front().unwrap();

            for mov in moves.clone() {
                if reduntant_move(mov, last_move) {
                    continue;
                }

                cube.cube_from_tuple(cube_i);
                cube.do_move(mov);

                if cube.is_solved() {
                    reverse_path.insert(cube.cube_to_tuple(), (cube_i, mov));

                    return get_path(starting_cube, reverse_path);
                }

                let cube_tuple = cube.cube_to_tuple();

                if !visited.contains(&cube_tuple) {
                    queue.push_back((cube.cube_to_tuple(), mov));
                    visited.insert(cube_tuple);
                    reverse_path.insert(cube.cube_to_tuple(), (cube_i, mov));
                }
            }
        }
    }
}

fn get_path(target_cube: &Cube, path_graph: HashMap<(u64, u64), ((u64, u64), Move)>) -> Vec<Move> {
    let mut path: Vec<Move> = vec![];

    let target = target_cube.cube_to_tuple();
    let mut m: Move;
    let mut k = Cube::new().cube_to_tuple();

    while k != target {
        let a = *path_graph.get(&k).unwrap();
        k = a.0;
        m = a.1;
        path.push(m);
    }

    path.reverse();

    path
}
