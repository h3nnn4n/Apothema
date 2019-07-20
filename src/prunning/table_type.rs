use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct PrunningTables {
    pub edge_orientation: HashMap<(u64, u64), u64>,
    pub edge_permutation: HashMap<(u64, u64), u64>,
    pub corner_orientation: HashMap<(u64, u64), u64>,
    pub corner_permutation: HashMap<(u64, u64), u64>,

    pub stats: Stats,
}

pub struct Stats {
    pub t_start: Instant,
    pub t_timer: Instant,
    pub t_delta: Duration,
    pub max_depth: u64,
}

impl PrunningTables {
    pub fn new() -> PrunningTables {
        PrunningTables{
            edge_orientation: HashMap::new(),
            edge_permutation: HashMap::new(),
            corner_orientation: HashMap::new(),
            corner_permutation: HashMap::new(),

            stats: Stats {
                t_start: Instant::now(),
                t_timer: Instant::now(),
                t_delta: Duration::from_millis(1000),
                max_depth: 0,
            }
        }
    }
}

impl Stats {
    pub fn start_timer(&mut self) {
        self.t_start = Instant::now();
        self.t_timer = self.t_start;
    }

    pub fn tick(&mut self) {
        let t_current = Instant::now();
        let t_diff = t_current.duration_since(self.t_timer);

        if t_diff > self.t_delta {
            self.t_timer = Instant::now();
            self.print_status();
        }
    }

    pub fn print_status(&self) {
        let t_current = Instant::now();
        let t_diff = t_current.duration_since(self.t_timer);

        // let time = (t_diff.as_secs() as f64) + (t_diff.subsec_millis() as f64 / 1000.0);
        // let moves_per_sec = (visited.len() as f64) / time;

        println!(
            "time_elapsed: {:4}.{:03}    depth: {:2}",
            t_diff.as_secs(),
            t_diff.subsec_millis(),
            self.max_depth,
        );
    }
}
