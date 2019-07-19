use std::time::{Instant};
use std::fs;
use ron;

use super::*;

pub fn store_table(prunning_tables: PrunningTables) {
    println!("Writing prunning table to disk");
    let t_start = Instant::now();

    let data = ron::ser::to_string(&prunning_tables.edge_orientation).unwrap();
    let data_len = data.len();
    fs::write("table.ron", data).expect("Unable to write file");

    let t_end = Instant::now();
    let t_diff = t_end.duration_since(t_start);

    println!(
        "Finished writting prunning table\ntime_elapsed {:4}.{:03}    file_size {} bytes",
        t_diff.as_secs(),
        t_diff.subsec_millis(),
        data_len,
    )
}
