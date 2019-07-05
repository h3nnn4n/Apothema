use super::*;
use num::FromPrimitive;

fn choose(n: u64, k: u64) -> u64 {
    let factorial = |x| (1..=x).fold(1, |a, x| a * x);
    factorial(n) / factorial(k) / factorial(n - k)
}

impl Cube {
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
