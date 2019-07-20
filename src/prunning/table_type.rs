use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::cube::Move;

pub struct PrunningTables {
    pub edge_orientation: HashMap<(u64, u64), u64>,
    pub edge_permutation: HashMap<(u64, u64), u64>,
    pub corner_orientation: HashMap<(u64, u64), u64>,
    pub corner_permutation: HashMap<(u64, u64), u64>,

    pub stats: Stats,
    pub state: State,
}

pub struct Stats {
    pub t_start: Instant,
    pub t_timer: Instant,
    pub t_delta: Duration,

    visited_nodes_size: VecDeque<(u64, Duration)>,
}

pub struct State {
    pub queue: VecDeque<((u64, u64), Move, u64)>,
    pub visited: HashSet<(u64, u64)>,
    pub max_depth: u64,
}

impl PrunningTables {
    pub fn new() -> PrunningTables {
        PrunningTables{
            edge_orientation: HashMap::new(),
            edge_permutation: HashMap::new(),
            corner_orientation: HashMap::new(),
            corner_permutation: HashMap::new(),

            stats: Stats::new(),
            state: State::new(),
        }
    }

    pub fn stats_tick(&mut self) {
        if self.stats.tick() {
            self.stats.update_move_per_sec(self.state.visited.len() as u64);
            self.print_status();
        }
    }

    pub fn print_status(&self) {
        let t_diff = self.stats.get_elapsed_time();

        println!(
            "time_elapsed: {:4}.{:03}    depth: {:2}    visited {:9} nodes    moves_per_sec: {:8.0}  table_size (EO,CO,EP,CP): {:5} {:5} {:9} {:6}",
            t_diff.as_secs(),
            t_diff.subsec_millis(),
            self.state.max_depth,
            self.state.visited.len(),
            self.stats.moves_per_sec(),
            self.edge_orientation.len(),
            self.corner_orientation.len(),
            self.edge_permutation.len(),
            self.corner_permutation.len(),
        );
    }
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            t_start: Instant::now(),
            t_timer: Instant::now(),
            t_delta: Duration::from_millis(1000),
            visited_nodes_size: VecDeque::new(),
        }
    }

    pub fn start_timer(&mut self) {
        self.t_start = Instant::now();
        self.t_timer = self.t_start;
    }

    pub fn tick(&mut self) -> bool {
        let t_current = Instant::now();
        let t_diff = t_current.duration_since(self.t_timer);

        if t_diff > self.t_delta {
            self.t_timer = Instant::now();
            return true
        }

        if self.visited_nodes_size.len() > 10 {
            self.visited_nodes_size.pop_front();
        }

        false
    }

    pub fn get_elapsed_time(&self) -> Duration {
        let t_current = Instant::now();
        let t_diff = t_current.duration_since(self.t_start);

        t_diff
    }

    pub fn update_move_per_sec(&mut self, nodes_size: u64) {
        let data = (nodes_size, self.get_elapsed_time());

        self.visited_nodes_size.push_back(data);
    }

    pub fn  moves_per_sec(&self) -> u64 {
        if self.visited_nodes_size.len() == 0 {
            return 0;
        }

        let front = self.visited_nodes_size.front().unwrap();
        let back = self.visited_nodes_size.back().unwrap();

        let duration_front = front.1;
        let duration_back = back.1;

        let duration_diff = duration_back - duration_front;

        let move_diff = (back.0 - front.0) as f64;

        let time = (duration_diff.as_secs() as f64) + (duration_diff.subsec_millis() as f64 / 1000.0);
        let moves_per_sec = move_diff / time;

        if time == 0.0 {
            return 0;
        }

        moves_per_sec as u64
    }
}

impl State {
    pub fn new() -> State {
        State {
            queue: VecDeque::new(),
            visited: HashSet::new(),
            max_depth: 0,
        }
    }
}