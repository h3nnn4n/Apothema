use super::{Corner, Corners, Edge, Edges, Move, OrientedCorner, OrientedEdge};
use phf::phf_map;

pub fn get_table_for(m: Move) -> (Edges, Corners) {
    let move_str = m.to_string();

    (EDGE_MOVE_TABLE[&*move_str], CORNER_MOVE_TABLE[&*move_str])
}

static EDGE_MOVE_TABLE: phf::Map<&'static str, Edges> = phf_map! {
    "Ux1" => [OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
              OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],
    "Rx1" => [OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::BR, o:0}, OrientedEdge{e:Edge::DF, o:0},
              OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UR, o:0}],
    "Fx1" => [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::FL, o:1}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:1},
              OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::UF, o:1}, OrientedEdge{e:Edge::DF, o:1}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],
    "Dx1" => [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DF, o:0}, OrientedEdge{e:Edge::DL, o:0},
              OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],
    "Lx1" => [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
              OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BR, o:0}],
    "Bx1" => [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::BR, o:1}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
              OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BL, o:1}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::UB, o:1}, OrientedEdge{e:Edge::DB, o:1}],
    "NOP" => [OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
              OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],
};

static CORNER_MOVE_TABLE: phf::Map<&'static str, Corners> = phf_map! {
    "Ux1" => [OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0},
              OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],
    "Rx1" => [OrientedCorner{c:Corner::DFR, o:2}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::URF, o:1},
              OrientedCorner{c:Corner::DRB, o:1}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::UBR, o:2}],
    "Fx1" => [OrientedCorner{c:Corner::UFL, o:1}, OrientedCorner{c:Corner::DLF, o:2}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::UBR, o:0},
              OrientedCorner{c:Corner::URF, o:2}, OrientedCorner{c:Corner::DFR, o:1}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],
    "Dx1" => [OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::UBR, o:0},
              OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}, OrientedCorner{c:Corner::DFR, o:0}],
    "Lx1" => [OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::ULB, o:1}, OrientedCorner{c:Corner::DBL, o:2}, OrientedCorner{c:Corner::UBR, o:0},
              OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::UFL, o:2}, OrientedCorner{c:Corner::DLF, o:1}, OrientedCorner{c:Corner::DRB, o:0}],
    "Bx1" => [OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::UBR, o:1}, OrientedCorner{c:Corner::DRB, o:2},
              OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::ULB, o:2}, OrientedCorner{c:Corner::DBL, o:1}],
    "NOP" => [OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::ULB, o:0},
              OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],
};
