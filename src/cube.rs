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
pub enum Move { Ux1, Ux2, Ux3, Rx1, Rx2, Rx3, Fx1, Fx2, Fx3, Dx1, Dx2, Dx3, Lx1, Lx2, Lx3, Bx1, Bx2, Bx3, NOP }

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
pub struct OrientedCorner {
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
        let (edges, corners) = move_table::get_table_for(Move::NOP);
        Cube {
            edges: edges,
            corners: corners,
        }
    }

    pub fn do_move(&mut self, m: Move) {
        match m {
            Move::Rx1 => self.move_r(),
            _ => panic!(),
        }
    }

    pub fn move_r(&mut self) {
        let (edge_move_table, corner_move_table) = &move_table::get_table_for(Move::Rx1);

        self.edge_multiply(edge_move_table);
        self.corner_multiply(corner_move_table);
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

        let corners = (0..8).into_iter().map(|key| Corner::from_u32(key).unwrap());

        for corner in corners.clone() {
            let inner_index = corner as usize;
            let outer_index = b[inner_index].c as usize;
            c_permutations[inner_index] = self.corners[outer_index].c;
            c_orientations[inner_index] = (b[outer_index].o + self.corners[outer_index].o) % 3;
        }

        for corner in corners {
            let index = corner as usize;
            self.corners[index].c = c_permutations[index];
            self.corners[index].o = c_orientations[index];
        }
    }
}

#[cfg(test)]
mod cube_tests;
