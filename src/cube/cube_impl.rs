use super::*;
use num::FromPrimitive;

impl Cube {
    pub fn new() -> Cube {
        let (edges, corners) = move_table::get_table_for(Move::NOP);
        Cube {
            edges: edges,
            corners: corners,
        }
    }

    pub fn is_solved(&self) -> bool {
        let (edges, corners) = move_table::get_table_for(Move::NOP);

        self.edges == edges && self.corners == corners
    }

    pub fn do_move(&mut self, m: Move) {
        let (edge_move_table, corner_move_table) = &move_table::get_table_for(m);

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
