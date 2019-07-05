use super::*;
use num::FromPrimitive;

impl Cube {
    pub fn cube_from_tuple(&mut self, (edge_i, corner_i): (u64, u64)) {
        self.edges_from_i(edge_i);
        self.corners_from_i(corner_i);
    }

    pub fn cube_to_tuple(&self) -> (u64, u64) {
        let edge_i = self.edges_to_i();
        let corner_i = self.corners_to_i();

        (edge_i, corner_i)
    }
}
