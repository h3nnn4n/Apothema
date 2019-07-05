use super::*;
use num::FromPrimitive;

fn choose(n: u64, k: u64) -> u64 {
    let factorial = |x| (1..=x).fold(1, |a, x| a * x);
    factorial(n) / factorial(k) / factorial(n - k)
}

impl Cube {
    pub fn sorted_ud_slice_as_u64(&self) -> u64 {
        let mut j: u64 = 0;
        let mut x: u64 = 0;
        let mut ud_edges = [Edge::UR; 4];

        let edges = (0..12).into_iter().map(|key| Edge::from_u32(key).unwrap());

        for edge_i in edges {
            let edge = self.edges[edge_i as usize].e;

            if edge == Edge::FR || edge == Edge::FL || edge == Edge::BL || edge == Edge::BR {
                ud_edges[j as usize] = edge;

                j += 1;
            }
        }

        // This loops j from 3 to 1 and k from j-1 to 0
        // Not the most readable
        for j in (1..4).rev() {
            let mut s: u64 = 0;

            for k in (0..j).rev() {
                if ud_edges[k] > ud_edges[j] {
                    s += 1;
                }
            }

            x = (x + s) * (j as u64);
        }

        self.ud_slice_as_u64() * 24 + x
    }

    pub fn ud_slice_as_u64(&self) -> u64 {
        let mut s: u64 = 0;
        let mut k: u64 = 3;
        let mut n: u64 = 11;
        let mut occupied = [false; 12];

        let edges = (0..12).into_iter().map(|key| Edge::from_u32(key).unwrap());

        for edge in edges {
            if self.edges[edge as usize].e >= Edge::FR {
                occupied[edge as usize] = true;
            }
        }

        loop {
            if occupied[n as usize] {
                if k == 0 {
                    break;
                }

                k -= 1;
            } else {
                s += choose(n, k);
                n -= 1;
            }
        }

        s
    }
}
