#[derive(Debug)]
pub enum Move {
    Ux1,
    Ux2,
    Ux3,
    Rx1,
    Rx2,
    Rx3,
    Fx1,
    Fx2,
    Fx3,
    Dx1,
    Dx2,
    Dx3,
    Lx1,
    Lx2,
    Lx3,
    Bx1,
    Bx2,
    Bx3,
    NOP,
}

enum_from_primitive! {
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Corner {
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
pub enum Edge {
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
    pub c: Corner,
    pub o: u32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct OrientedEdge {
    pub e: Edge,
    pub o: u32,
}

pub struct Cube {
    pub corners: [OrientedCorner; 8],
    pub edges: [OrientedEdge; 12],
}

pub type Edges = [OrientedEdge; 12];
pub type Corners = [OrientedCorner; 8];
