#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused)]

use serde::{Deserialize, Serialize};
use thiserror::Error;
use nanoid::nanoid;

pub mod comm;
mod config;
mod core;
pub mod interface;
mod log;
mod metal;
mod state;
mod storage;

#[derive(Error, Debug)]
pub enum NodeErr {
    #[error("No peers given")]
    NoPeers,

    #[error("Number of peers must be even")]
    InvalidPeerCount,

    #[error("unknown error")]
    Unknown,
}

pub struct Client;

struct Mod {
    comm: comm::Handle,
    interface: interface::Handle,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModConfigData {
    pub comm: comm::Config,
    pub interface: interface::Config,
}

pub struct Node {
    raft_node_id: String,   // if there are 3 raft nodes, each of them should have a static id (e.g. n1/n2/n3)
                            // so clashes could happen if e.g. two processes with the same id are running
    ephemeral_id: String,   // so we add an ephemeral runtime id (random generated on start) and combine it with the raft_node_id,
                            // that allows us to auto kill self if e.g. lex id < than other

    mods: Mod,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeConfigData {
    pub raft_node_id: String,

    pub mods: ModConfigData,
}

impl Node {
    pub fn new(conf_data: NodeConfigData) -> Self {
        let mut cwd = std::env::current_dir().expect("Could not obtain current working dir");
        
        cwd.push("state.json");

        metal::write(&conf_data, cwd).expect("Could not save state.json");

        Self::configure(conf_data)
    }

    pub fn load(path_file: std::path::PathBuf) -> Self {
        let res = metal::read::<NodeConfigData>(path_file).expect("Could not load file");
        Self::configure(res)
    }

    pub async fn run(&self) -> Result<(), NodeErr>
    {
        Err(NodeErr::Unknown)
    }

    fn configure(config: NodeConfigData) -> Self {
        tracing::info!("Using conf:");
        tracing::info!("{config:#?}");
        let e_id = nanoid!(32);
        tracing::info!("Using ephemeral id: {e_id}", );

        Self {
            raft_node_id: config.raft_node_id,
            ephemeral_id: e_id,
            mods: Mod { 
                comm: comm::Handle::new(config.mods.comm),
                interface: interface::Handle::new(config.mods.interface)
            }
        }
    }
}