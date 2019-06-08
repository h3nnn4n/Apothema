use super::*;

pub fn reverse_move_sequence(move_sequence: Moves) -> Moves {
    move_sequence
        .into_iter()
        .map(|m| reverse_move(m))
        .rev()
        .collect()
}

pub fn find_move_sequence_diameter(move_sequence: &Moves) -> u32 {
    let mut cube = Cube::new();
    let solved_cube = Cube::new();
    let mut count = 0;

    loop {
        cube.do_move_sequence(&move_sequence);
        count += 1;

        if cube == solved_cube {
            break;
        }
    }

    count
}
