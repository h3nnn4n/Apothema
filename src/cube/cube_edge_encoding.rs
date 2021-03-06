use super::*;
use num::FromPrimitive;

impl Cube {
    pub fn edge_orientation_as_u64(&self) -> u64 {
        let mut o: u64 = 0;

        for (k, v) in self.edges.iter().enumerate() {
            o += ((v.o) as u64) * 2_u64.pow(k as u32);
        }

        o
    }

    pub fn edge_permutation_as_u64(&self) -> u64 {
        let mut p: u64 = 0;

        for (k, v) in self.edges.iter().enumerate() {
            p += ((v.e) as u64) * 12_u64.pow(k as u32);
        }

        p
    }

    pub fn edges_from_tuple(&mut self, (_p, _o): (u64, u64)) {
        let mut p = _p;
        let mut o = _o;

        for index in 0..12 {
            self.edges[index].e = Edge::from_u32((p % 12) as u32).unwrap();
            self.edges[index].o = (o % 2) as u32;

            p /= 12;
            o /= 2;
        }
    }

    pub fn edges_to_tuple(&self) -> (u64, u64) {
        let mut p: u64 = 0;
        let mut o: u64 = 0;

        for (k, v) in self.edges.iter().enumerate() {
            p += ((v.e) as u64) * 12_u64.pow(k as u32);
            o += ((v.o) as u64) * 2_u64.pow(k as u32);
        }

        (p, o)
    }

    pub fn edges_from_i(&mut self, i: u64) {
        let o: u64 = i % 4096;
        let p: u64 = i / 4096;

        self.edges_from_tuple((p, o));
    }

    pub fn edges_to_i(&self) -> u64 {
        let (p, o) = self.edges_to_tuple();

        p * 4096 + o
    }
}
