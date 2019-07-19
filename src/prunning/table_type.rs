use std::collections::HashMap;

pub struct PrunningTables {
    pub edge_orientation: HashMap<(u64, u64), u64>,
    pub edge_permutation: HashMap<(u64, u64), u64>,
    pub corner_orientation: HashMap<(u64, u64), u64>,
    pub corner_permutation: HashMap<(u64, u64), u64>,
}

impl PrunningTables {
    pub fn new() -> PrunningTables {
        PrunningTables{
            edge_orientation: HashMap::new(),
            edge_permutation: HashMap::new(),
            corner_orientation: HashMap::new(),
            corner_permutation: HashMap::new(),
        }
    }
}