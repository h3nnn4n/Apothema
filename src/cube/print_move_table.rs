use super::*;

const MOVES: [Move; 19] = [
    Move::NOP,
    Move::Rx1,
    Move::Rx2,
    Move::Rx3,
    Move::Lx1,
    Move::Lx2,
    Move::Lx3,
    Move::Fx1,
    Move::Fx2,
    Move::Fx3,
    Move::Bx1,
    Move::Bx2,
    Move::Bx3,
    Move::Ux1,
    Move::Ux2,
    Move::Ux3,
    Move::Dx1,
    Move::Dx2,
    Move::Dx3,
];

pub fn print_edge_move_table() {
    for m in MOVES.iter() {
        let mut cube = Cube::new();

        cube.do_move(*m);
        print_edges(*m, &cube.edges);
    }
}

pub fn print_corner_move_table() {
    for m in MOVES.iter() {
        let mut cube = Cube::new();

        cube.do_move(*m);
        print_corners(*m, &cube.corners);
    }
}

fn print_edges(m: Move, edges: &Edges) {
    println!("    \"{}\" => [OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}},
              OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}, OrientedEdge{{e: Edge::{}, o: {}}}],",
        m,
        edges[0].e, edges[0].o,
        edges[1].e, edges[1].o,
        edges[2].e, edges[2].o,
        edges[3].e, edges[3].o,
        edges[4].e, edges[4].o,
        edges[5].e, edges[5].o,
        edges[6].e, edges[6].o,
        edges[7].e, edges[7].o,
        edges[8].e, edges[8].o,
        edges[9].e, edges[9].o,
        edges[10].e, edges[10].o,
        edges[11].e, edges[11].o,
    )
}

fn print_corners(m: Move, corners: &Corners) {
    println!("    \"{}\" => [OrientedCorner{{c: Corner::{}, o: {}}}, OrientedCorner{{c: Corner::{}, o: {}}}, OrientedCorner{{c: Corner::{}, o: {}}}, OrientedCorner{{c: Corner::{}, o: {}}},
              OrientedCorner{{c: Corner::{}, o: {}}}, OrientedCorner{{c: Corner::{}, o: {}}}, OrientedCorner{{c: Corner::{}, o: {}}}, OrientedCorner{{c: Corner::{}, o: {}}}],",
        m,
        corners[0].c, corners[0].o,
        corners[1].c, corners[1].o,
        corners[2].c, corners[2].o,
        corners[3].c, corners[3].o,
        corners[4].c, corners[4].o,
        corners[5].c, corners[5].o,
        corners[6].c, corners[6].o,
        corners[7].c, corners[7].o,
    )
}
