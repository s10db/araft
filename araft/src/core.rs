use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use nanoid::nanoid;

pub mod comm;
pub mod interface;
pub mod state;
mod log;
mod metal;
mod storage;

#[derive(Error, Debug)]
pub enum NodeErr {
    #[error("no peers given")]
    NoPeers,

    #[error("number of peers must be even")]
    InvalidPeerCount,

    #[error("unknown error")]
    Unknown,
}

pub struct Client;

struct Mod {
    comm: comm::Handle,
    interface: Option<interface::Handle>,
    state: state::Handle,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModConfigData {
    pub comm: comm::Config,
    pub interface: interface::Config,
    pub state: state::Config,
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
        let mut cwd = std::env::current_dir().expect("could not obtain current working dir");
        
        cwd.push("state.json");

        metal::write(&conf_data, cwd).expect("could not save state.json");

        Self::configure(conf_data)
    }

    pub fn load(path_file: std::path::PathBuf) -> Self {
        let res = metal::read::<NodeConfigData>(path_file).expect("could not load file");
        Self::configure(res)
    }

    pub async fn run(mut self) -> Result<(), NodeErr> {
        let Some(interface) = self.mods.interface else {
            panic!("interface can't be None");
        };
        tokio::spawn(async move {
            interface.run().await
        });

        self.mods.interface = None;

        self.main_loop().await
    }

    async fn main_loop(&self) -> Result<(), NodeErr> {
        match self.mods.state.role() {
            state::Role::Undef => panic!("Invalid role"),
            state::Role::Follower => {
               
            },
            state::Role::Candidate => todo!(),
            state::Role::Leader => todo!(),
        }

        Err(NodeErr::Unknown)
    }

    fn configure(config: NodeConfigData) -> Self {
        tracing::info!("using conf:");
        tracing::info!("{config:#?}");
        let e_id = nanoid!(32);
        tracing::info!("using ephemeral id: {e_id}", );

        Self {
            raft_node_id: config.raft_node_id,
            ephemeral_id: e_id,
            mods: Mod { 
                comm: comm::Handle::new(config.mods.comm),
                interface: Some(interface::Handle::new(config.mods.interface)),
                state: state::Handle::new(config.mods.state)
            }
        }
    }
}