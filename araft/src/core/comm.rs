use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub peers: Vec<String>,
}

pub struct Handle {
    config: Config
}

impl Handle {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}