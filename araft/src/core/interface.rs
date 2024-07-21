use serde::{Deserialize, Serialize};

use tonic::{transport::Server, Request, Response, Status};

pub mod raft {
    tonic::include_proto!("raft");
}

use raft::raft_server::{Raft, RaftServer};
use raft::{CommandsIn, CommandsOut, EchoIn, EchoOut};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub cli_addr: std::net::SocketAddr,
    // pub cmd_bind_addr: String,
    // pub cmd_bind_port: u16,
}

struct RaftRpc;

pub struct Handle {
    config: Config
}

impl Handle {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<(), tonic::transport::Error> {
        tracing::info!("starting interface");
        let srv = RaftRpc;
        Server::builder().add_service(RaftServer::new(srv)).serve(self.config.cli_addr).await?;
        tracing::info!("closing interface");
        Ok(())
    }
}

#[tonic::async_trait]
impl Raft for RaftRpc {
    async fn command(&self, request: Request<CommandsIn>) -> Result<Response<CommandsOut>, Status> {
        unimplemented!()
    }

    async fn echo(&self, request: Request<EchoIn>) -> Result<Response<EchoOut>, Status> {
        let resp = EchoOut {
            message: request.into_inner().name
        };

        Ok(Response::new(resp))
    }
}