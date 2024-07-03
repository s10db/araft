#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused)]

use thiserror::Error;

mod comm;
mod config;
mod core;
mod interface;
mod log;
mod metal;
mod state;

#[derive(Error, Debug)]
pub enum Err {
    #[error("No peers given")]
    NoPeers,

    #[error("Number of peers must be even")]
    InvalidPeerCount,

    #[error("unknown error")]
    Unknown,
}

pub struct Node {

}

pub struct Client;

impl Node {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self,
                     bind: impl Into<String>, 
                     port: u16,
                     id: impl Into<String>,
                     peers: Vec<String>) -> Result<(), Err>
    {
        if peers.is_empty() {
            return Err(Err::NoPeers);
        }

        if peers.len() % 2 != 0 {
            return Err(Err::InvalidPeerCount);
        }

        Err(Err::Unknown)
    }
}