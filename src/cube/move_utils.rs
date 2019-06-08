use super::*;

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
