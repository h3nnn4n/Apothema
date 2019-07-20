use std::collections::HashMap;
use std::time::{Instant};
use std::fs;
use ron;

use super::*;

pub fn store_table(prunning_tables: PrunningTables) {
    println!("Writing prunning tables to disk");
    let t_start = Instant::now();

    write_table(&prunning_tables.edge_orientation, "edge_orientation.ron".to_string());
    write_table(&prunning_tables.corner_orientation, "corner_orientation.ron".to_string());
    write_table(&prunning_tables.edge_permutation, "edge_permutation.ron".to_string());
    write_table(&prunning_tables.corner_permutation, "corner_permutation.ron".to_string());

    let t_end = Instant::now();
    let t_diff = t_end.duration_since(t_start);

    println!(
        "Finished writting prunning tables\ntime_elapsed {:4}.{:03}",
        t_diff.as_secs(),
        t_diff.subsec_millis(),
    )
}

fn write_table(table: &HashMap<(u64, u64), u64>, name: String) {
    let data = ron::ser::to_string(table).unwrap();
    fs::write(name, data).expect("Unable to write file");
}
