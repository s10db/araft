#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused)]

mod config;
mod core;

pub use core::{NodeConfigData, ModConfigData};
pub use core::comm::Config as CommConfig;
pub use core::interface::Config as InterfaceConfig;
pub use core::state::Config as StateConfig;

pub struct Client;

pub struct Node {
    core: core::Node
}

impl Node {
    pub fn new(conf_data: core::NodeConfigData) -> Self {
        Self {
            core: core::Node::new(conf_data)
        }
    }

    pub fn load(path_file: std::path::PathBuf) -> Self {
        Self {
            core: core::Node::load(path_file)
        }
    }

    pub async fn run(self)
    {
        tracing::info!("starting raft node");

        let res = self.core.run().await;
        match res {
            Ok(_) => {
                tracing::info!("closing raft node gracefully");
            },
            Err(_) => {
                tracing::error!("closing raft node with error");
            },
        }
    }
}