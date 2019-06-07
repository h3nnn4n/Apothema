#![plugin(phf_macros)]
use phf::phf_map;

use super::{Edge, Edges, Move, OrientedEdge};

pub fn get_table_for(m: Move) -> Edges {
    let move_str = m.to_string();
    EDGE_MOVE_TABLE[&*move_str]
}

static EDGE_MOVE_TABLE: phf::Map<&'static str, Edges> = phf_map! {
    "Rx1" => [OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::BR, o:0}, OrientedEdge{e:Edge::DF, o:0},
              OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UR, o:0}],
};
