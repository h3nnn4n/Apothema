#![allow(dead_code)]

enum Face {
    F1, F2, F3, F4, F5, F6, F7, F8, F9,
    B1, B2, B3, B4, B5, B6, B7, B8, B9,
    U1, U2, U3, U4, U5, U6, U7, U8, U9,
    D1, D2, D3, D4, D5, D6, D7, D8, D9,
    L1, L2, L3, L4, L5, L6, L7, L8, L9,
    R1, R2, R3, R4, R5, R6, R7, R8, R9,
}
enum TurnAxis { U, R, F, D, L, B }
pub enum Move { Ux1, Ux2, Ux3, Rx1, Rx2, Rx3, Fx1, Fx2, Fx3, Dx1, Dx2, Dx3, Lx1, Lx2, Lx3, Bx1, Bx2, Bx3 }
enum ColorIndex { UCol, RCol, FCol, DCol, LCol, BCol, NoCol }
enum Corner { URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB }
enum Edge { UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR }

struct OrientedCorner {
    c: Corner,
    o: u32,
}

struct OrientedEdge {
    e: Edge,
    o: u32,
}

pub struct Cube {
    corners: Vec<OrientedCorner>,
    edges: Vec<OrientedEdge>,
    corner_cubie_move_table: Vec<Vec<OrientedCorner>>,
    edge_cubie_move_table: Vec<Vec<OrientedEdge>>,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            edges: init_edges(),
            corners: init_corners(),
            corner_cubie_move_table: init_corner_cubie_move(),
            edge_cubie_move_table: init_edge_cubie_move(),
        }
    }

    pub fn do_move(&mut self, m: Move) {
        match m {
            _ => ()
        }
    }
}

fn init_corners() -> Vec<OrientedCorner> {
    vec![
        OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::UBR, o:0},
        OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}
    ]
}

fn init_edges() -> Vec<OrientedEdge> {
    vec![
        OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
        OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}
    ]
}

fn init_corner_cubie_move() -> Vec<Vec<OrientedCorner>> {
    vec![
        vec![OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0},
            OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],  //U
        vec![OrientedCorner{c:Corner::DFR, o:2}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::URF, o:1},
            OrientedCorner{c:Corner::DRB, o:1}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::UBR, o:2}],  //R
        vec![OrientedCorner{c:Corner::UFL, o:1}, OrientedCorner{c:Corner::DLF, o:2}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::UBR, o:0},
            OrientedCorner{c:Corner::URF, o:2}, OrientedCorner{c:Corner::DFR, o:1}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],  //F
        vec![OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::UBR, o:0},
            OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}, OrientedCorner{c:Corner::DFR, o:0}],  //D
        vec![OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::ULB, o:1}, OrientedCorner{c:Corner::DBL, o:2}, OrientedCorner{c:Corner::UBR, o:0},
            OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::UFL, o:2}, OrientedCorner{c:Corner::DLF, o:1}, OrientedCorner{c:Corner::DRB, o:0}],  //L
        vec![OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::UBR, o:1}, OrientedCorner{c:Corner::DRB, o:2},
            OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::ULB, o:2}, OrientedCorner{c:Corner::DBL, o:1}],  //B
        vec![OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0},
            OrientedCorner{c:Corner::DRB, o:0}, OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}],  //Us
        vec![OrientedCorner{c:Corner::DFR, o:2}, OrientedCorner{c:Corner::DLF, o:1}, OrientedCorner{c:Corner::UFL, o:2}, OrientedCorner{c:Corner::URF, o:1},
            OrientedCorner{c:Corner::DRB, o:1}, OrientedCorner{c:Corner::DBL, o:2}, OrientedCorner{c:Corner::ULB, o:1}, OrientedCorner{c:Corner::UBR, o:2}],  //Rs
        vec![OrientedCorner{c:Corner::UFL, o:1}, OrientedCorner{c:Corner::DLF, o:2}, OrientedCorner{c:Corner::DBL, o:1}, OrientedCorner{c:Corner::ULB, o:2},
            OrientedCorner{c:Corner::URF, o:2}, OrientedCorner{c:Corner::DFR, o:1}, OrientedCorner{c:Corner::DRB, o:2}, OrientedCorner{c:Corner::UBR, o:1}]   //Fs
    ]
}

fn init_edge_cubie_move() -> Vec<Vec<OrientedEdge>> {
    vec![
        vec![OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //U
        vec![OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::BR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UR, o:0}],  //R
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::FL, o:1}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:1},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::UF, o:1}, OrientedEdge{e:Edge::DF, o:1}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //F
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DF, o:0}, OrientedEdge{e:Edge::DL, o:0},
            OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //D
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //L
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::BR, o:1}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BL, o:1}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::UB, o:1}, OrientedEdge{e:Edge::DB, o:1}],  //B
        vec![OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0},
            OrientedEdge{e:Edge::DF, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //Us
        vec![OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::BR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UR, o:0}],  //Rs
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::FL, o:1}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::BL, o:1}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:1},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BR, o:1}, OrientedEdge{e:Edge::UF, o:1}, OrientedEdge{e:Edge::DF, o:1}, OrientedEdge{e:Edge::DB, o:1}, OrientedEdge{e:Edge::UB, o:1}]   //Fs
    ]
}
