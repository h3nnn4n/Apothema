use super::{Cube, Move, Moves};
use rand::seq::SliceRandom;
use rand::thread_rng;

const MOVES: [Move; 18] = [
    Move::Rx1,
    Move::Rx2,
    Move::Rx3,
    Move::Lx1,
    Move::Lx2,
    Move::Lx3,
    Move::Ux1,
    Move::Ux2,
    Move::Ux3,
    Move::Dx1,
    Move::Dx2,
    Move::Dx3,
    Move::Fx1,
    Move::Fx2,
    Move::Fx3,
    Move::Bx1,
    Move::Bx2,
    Move::Bx3,
];

pub fn get_random_move() -> Move {
    let mut rng = thread_rng();

    *MOVES.choose(&mut rng).unwrap()
}

pub fn get_random_move_sequence(lenght: usize) -> Moves {
    let mut rng = thread_rng();

    MOVES.choose_multiple(&mut rng, lenght).cloned().collect()
}

pub fn reverse_move(m: Move) -> Move {
    match m {
        Move::Rx1 => Move::Rx3,
        Move::Rx2 => Move::Rx2,
        Move::Rx3 => Move::Rx1,

        Move::Lx1 => Move::Lx3,
        Move::Lx2 => Move::Lx2,
        Move::Lx3 => Move::Lx1,

        Move::Ux1 => Move::Ux3,
        Move::Ux2 => Move::Ux2,
        Move::Ux3 => Move::Ux1,

        Move::Dx1 => Move::Dx3,
        Move::Dx2 => Move::Dx2,
        Move::Dx3 => Move::Dx1,

        Move::Fx1 => Move::Fx3,
        Move::Fx2 => Move::Fx2,
        Move::Fx3 => Move::Fx1,

        Move::Bx1 => Move::Bx3,
        Move::Bx2 => Move::Bx2,
        Move::Bx3 => Move::Bx1,

        Move::NOP => Move::NOP,
    }
}

pub fn find_move_diameter(m: Move) -> u32 {
    let mut cube = Cube::new();

    let mut counter = 0;

    loop {
        cube.do_move(m);
        counter += 1;

        if cube.is_solved() {
            break;
        }
    }

    counter
}
