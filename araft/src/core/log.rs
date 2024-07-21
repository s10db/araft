use ahash::{HashMap, HashMapExt};

pub mod raft {
    tonic::include_proto!("raft");
}

use raft::CommandsIn;

pub struct Log {
    // all the code in this project... only for keeping this map in sync on multiple machines:
    data: HashMap<u64, (u64, CommandsIn)>, // logIndex/(logTerm, Entry)
}

impl Log {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, index: u64, entry: (u64, CommandsIn)) {
        self.data.insert(index, entry);
    }

    pub fn check_append_valid(&self, prev_log_index: u64, prev_log_term: u64) -> bool {
        match self.data.get(&prev_log_index) {
            Some(val) => (val.0 == prev_log_term),
            None => false
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn latest_term(&self) -> u64 {
        if self.data.is_empty() {
            return 0;
        }
        let len = u64::try_from(self.data.len()).unwrap() - 1;
        match self.data.get(&len) {
            Some(v) => v.0,
            None => 0
        }
    }
}