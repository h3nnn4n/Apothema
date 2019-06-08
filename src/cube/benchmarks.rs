use super::{get_random_move, Cube, Move};
use std::time::Instant;

const MOVE_GRANULARITY: u64 = 1000;
const TIME_GRANULARITY: f64 = 0.5;
const N_REPS: u32 = 5;

pub fn fixed_move_per_sec() -> u64 {
    let mut total_move_count = 0;
    let mut total_elapsed_time = 0;

    for _ in 0..N_REPS {
        let start_time = Instant::now();
        let mut cube = Cube::new();
        let mut elapsed_time;
        let mut move_count = 0;

        loop {
            for _ in 0..MOVE_GRANULARITY {
                cube.do_move(Move::Rx1);
            }

            move_count += MOVE_GRANULARITY;

            elapsed_time = start_time.elapsed().as_millis();

            if elapsed_time > (TIME_GRANULARITY * 1000f64) as u128 {
                break;
            }
        }

        total_move_count += move_count;
        total_elapsed_time += elapsed_time;
    }

    let moves_per_millisec = (total_move_count as f64) / (total_elapsed_time as f64);

    (moves_per_millisec * 1000_f64) as u64
}

pub fn random_move_per_sec() -> u64 {
    let mut total_move_count = 0;
    let mut total_elapsed_time = 0;

    for _ in 0..N_REPS {
        let start_time = Instant::now();
        let mut cube = Cube::new();
        let mut elapsed_time;
        let mut move_count = 0;

        loop {
            for _ in 0..MOVE_GRANULARITY {
                let m = get_random_move();
                cube.do_move(m);
            }

            move_count += MOVE_GRANULARITY;

            elapsed_time = start_time.elapsed().as_millis();

            if elapsed_time > (TIME_GRANULARITY * 1000f64) as u128 {
                break;
            }
        }

        total_move_count += move_count;
        total_elapsed_time += elapsed_time;
    }

    let moves_per_millisec = (total_move_count as f64) / (total_elapsed_time as f64);

    (moves_per_millisec * 1000_f64) as u64
}
