#![allow(dead_code)]
mod move_table;

use num::FromPrimitive;
use std::fmt;

#[rustfmt::skip]
enum Face {
    F1, F2, F3, F4, F5, F6, F7, F8, F9,
    B1, B2, B3, B4, B5, B6, B7, B8, B9,
    U1, U2, U3, U4, U5, U6, U7, U8, U9,
    D1, D2, D3, D4, D5, D6, D7, D8, D9,
    L1, L2, L3, L4, L5, L6, L7, L8, L9,
    R1, R2, R3, R4, R5, R6, R7, R8, R9,
}

#[rustfmt::skip]
enum TurnAxis { U, R, F, D, L, B }

#[rustfmt::skip]
#[derive(Debug)]
pub enum Move { Ux1, Ux2, Ux3, Rx1, Rx2, Rx3, Fx1, Fx2, Fx3, Dx1, Dx2, Dx3, Lx1, Lx2, Lx3, Bx1, Bx2, Bx3 }

#[rustfmt::skip]
enum ColorIndex { UCol, RCol, FCol, DCol, LCol, BCol, NoCol }

enum_from_primitive! {
#[derive(Copy, Clone, PartialEq, Debug)]
enum Corner {
    URF,
    UFL,
    ULB,
    UBR,
    DFR,
    DLF,
    DBL,
    DRB,
}}

enum_from_primitive! {
#[derive(Copy, Clone, PartialEq, Debug)]
enum Edge {
    UR,
    UF,
    UL,
    UB,
    DR,
    DF,
    DL,
    DB,
    FR,
    FL,
    BL,
    BR,
}}

#[derive(Copy, Clone, PartialEq, Debug)]
struct OrientedCorner {
    c: Corner,
    o: u32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct OrientedEdge {
    e: Edge,
    o: u32,
}

pub struct Cube {
    corners: [OrientedCorner; 8],
    edges: [OrientedEdge; 12],
}

type Edges = [OrientedEdge; 12];
type Corners = [OrientedCorner; 8];

// Impl

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl fmt::Display for OrientedEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.e, self.o)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            edges: init_edges(),
            corners: init_corners(),
        }
    }

    pub fn do_move(&mut self, m: Move) {
        match m {
            Move::Rx1 => self.move_r(),
            _ => panic!()
        }
    }

    pub fn move_r(&mut self) {
        let edge_move_table = &move_table::get_table_for(Move::Rx1);

        self.edge_multiply(edge_move_table);
    }

    fn edge_multiply(&mut self, b: &Edges) {
        let mut e_permutations: [Edge; 12] = [Edge::UR; 12];
        let mut e_orientations: [u32; 12] = [0; 12];

        let edges = (0..12).into_iter().map(|key| Edge::from_u32(key).unwrap());

        for edge in edges.clone() {
            let inner_index = edge as usize;
            let outer_index = b[inner_index].e as usize;
            e_permutations[inner_index] = self.edges[outer_index].e;
            e_orientations[inner_index] = (b[outer_index].o + self.edges[outer_index].o) % 2;
        }

        for edge in edges {
            let index = edge as usize;
            self.edges[index].e = e_permutations[index];
            self.edges[index].o = e_orientations[index];
        }
    }

    fn corner_multiply(&mut self, b: &Corners) {
        let mut c_permutations: [Corner; 8] = [Corner::URF; 8];
        let mut c_orientations: [u32; 8] = [0; 8];

        let corners = (0..12).into_iter().map(|key| Corner::from_u32(key).unwrap());

        for corner in corners.clone() {
            let inner_index = corner as usize;
            let outer_index = b[inner_index].c as usize;
            c_permutations[inner_index] = self.corners[outer_index].c;
            c_orientations[inner_index] = (b[outer_index].o + self.corners[outer_index].o) % 2;
        }

        for corner in corners {
            let index = corner as usize;
            self.corners[index].c = c_permutations[index];
            self.corners[index].o = c_orientations[index];
        }
    }
}

fn move_index(m: Move) -> usize {
    match m {
        Move::Ux1 | Move::Ux2 | Move::Ux3 => 0,
        Move::Rx1 | Move::Rx2 | Move::Rx3 => 1,
        Move::Fx1 | Move::Fx2 | Move::Fx3 => 2,
        Move::Dx1 | Move::Dx2 | Move::Dx3 => 3,
        Move::Lx1 | Move::Lx2 | Move::Lx3 => 4,
        Move::Bx1 | Move::Bx2 | Move::Bx3 => 5,
    }
}

fn init_corners() -> Corners {
    [
        OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::ULB, o:0},
        OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}
    ]
}

fn init_edges() -> Edges {
    [
        OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
        OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}
    ]
}

fn corner_cubie_move() -> [Corners; 9] {
    [
        [OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0},
         OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],  //U
        [OrientedCorner{c:Corner::DFR, o:2}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::URF, o:1},
         OrientedCorner{c:Corner::DRB, o:1}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::UBR, o:2}],  //R
        [OrientedCorner{c:Corner::UFL, o:1}, OrientedCorner{c:Corner::DLF, o:2}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::UBR, o:0},
         OrientedCorner{c:Corner::URF, o:2}, OrientedCorner{c:Corner::DFR, o:1}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],  //F
        [OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::UBR, o:0},
         OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}, OrientedCorner{c:Corner::DFR, o:0}],  //D
        [OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::ULB, o:1}, OrientedCorner{c:Corner::DBL, o:2}, OrientedCorner{c:Corner::UBR, o:0},
         OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::UFL, o:2}, OrientedCorner{c:Corner::DLF, o:1}, OrientedCorner{c:Corner::DRB, o:0}],  //L
        [OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::UBR, o:1}, OrientedCorner{c:Corner::DRB, o:2},
         OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::ULB, o:2}, OrientedCorner{c:Corner::DBL, o:1}],  //B
        [OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0},
         OrientedCorner{c:Corner::DRB, o:0}, OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}],  //Us
        [OrientedCorner{c:Corner::DFR, o:2}, OrientedCorner{c:Corner::DLF, o:1}, OrientedCorner{c:Corner::UFL, o:2}, OrientedCorner{c:Corner::URF, o:1},
         OrientedCorner{c:Corner::DRB, o:1}, OrientedCorner{c:Corner::DBL, o:2}, OrientedCorner{c:Corner::ULB, o:1}, OrientedCorner{c:Corner::UBR, o:2}],  //Rs
        [OrientedCorner{c:Corner::UFL, o:1}, OrientedCorner{c:Corner::DLF, o:2}, OrientedCorner{c:Corner::DBL, o:1}, OrientedCorner{c:Corner::ULB, o:2},
         OrientedCorner{c:Corner::URF, o:2}, OrientedCorner{c:Corner::DFR, o:1}, OrientedCorner{c:Corner::DRB, o:2}, OrientedCorner{c:Corner::UBR, o:1}]   //Fs
    ]
}

fn edge_cubie_move() -> [Edges; 9] {
    [
        [OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
         OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //U
        [OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::BR, o:0}, OrientedEdge{e:Edge::DF, o:0},
         OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UR, o:0}],  //R
        [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::FL, o:1}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:1},
         OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::UF, o:1}, OrientedEdge{e:Edge::DF, o:1}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //F
        [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DF, o:0}, OrientedEdge{e:Edge::DL, o:0},
         OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //D
        [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
         OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //L
        [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::BR, o:1}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
         OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BL, o:1}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::UB, o:1}, OrientedEdge{e:Edge::DB, o:1}],  //B
        [OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0},
         OrientedEdge{e:Edge::DF, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //Us
        [OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::BR, o:0}, OrientedEdge{e:Edge::DF, o:0},
         OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UR, o:0}],  //Rs
        [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::FL, o:1}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::BL, o:1}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:1},
         OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BR, o:1}, OrientedEdge{e:Edge::UF, o:1}, OrientedEdge{e:Edge::DF, o:1}, OrientedEdge{e:Edge::DB, o:1}, OrientedEdge{e:Edge::UB, o:1}]   //Fs
    ]
}


#[cfg(test)]
mod cube_tests;
