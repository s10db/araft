use ahash::{HashMap, HashMapExt};

pub struct Storage {
    kvs: HashMap<String, String>, // logIndex/(logTerm, Entry)
}