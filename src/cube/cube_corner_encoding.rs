use super::*;
use num::FromPrimitive;

impl Cube {
    pub fn corners_from_tuple(&mut self, (_p, _o): (u64, u64)) {
        let mut p = _p;
        let mut o = _o;

        for index in 0..8 {
            self.corners[index].c = Corner::from_u32((p % 8) as u32).unwrap();
            self.corners[index].o = (o % 3) as u32;

            p /= 8;
            o /= 3;
        }
    }

    pub fn corner_orientation_as_u64(&self) -> u64 {
        let mut o: u64 = 0;

        for (k, v) in self.corners.iter().enumerate() {
            o += ((v.o) as u64) * 3_u64.pow(k as u32);
        }

        o
    }

    pub fn corner_permutation_as_u64(&self) -> u64 {
        let mut p: u64 = 0;

        for (k, v) in self.corners.iter().enumerate() {
            p += ((v.c) as u64) * 8_u64.pow(k as u32);
        }

        p
    }

    pub fn corners_to_tuple(&self) -> (u64, u64) {
        let mut p: u64 = 0;
        let mut o: u64 = 0;

        for (k, v) in self.corners.iter().enumerate() {
            p += ((v.c) as u64) * 8_u64.pow(k as u32);
            o += ((v.o) as u64) * 3_u64.pow(k as u32);
        }

        (p, o)
    }

    pub fn corners_from_i(&mut self, i: u64) {
        let o: u64 = i % 6561;
        let p: u64 = i / 6561;

        self.corners_from_tuple((p, o));
    }

    pub fn corners_to_i(&self) -> u64 {
        let (p, o) = self.corners_to_tuple();

        p * 6561 + o
    }
}
