use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cli_bind_addr: String,
    pub cli_bind_port: u16,
    // pub cmd_bind_addr: String,
    // pub cmd_bind_port: u16,
}

pub struct Handle {
    config: Config
}

impl Handle {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}