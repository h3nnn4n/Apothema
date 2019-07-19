use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fs;
use ron;

pub fn store_table(table: HashMap<(u64, u64), u64>) {
    println!("Writing prunning table to disk");
    let t_start = Instant::now();

    let data = ron::ser::to_string(&table).unwrap();
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
