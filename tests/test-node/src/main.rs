#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused)]

use clap::Parser;
// use s10_raft::Raft;

#[derive(Parser, Debug)]
struct Args {
    /// raft node id
    #[arg(short, long)]
    id: String,

    /// rpc bind address
    #[arg(short = 'a', long, default_value="[::1]")]
    rpc_bind_addr: String,

    /// rpc bind port
    #[arg(short = 'p', long)]
    rpc_bind_port: u16,

    /// list of peers to connect to
    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    peers: Vec<String>,
}

async fn run(args: Args) {
    // let mut raft = Raft::init().await;
    // raft.run_raft_node(args.rpc_bind_addr, args.rpc_bind_port, args.id, args.peers).await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const BIN_NAME: &'static str = env!("CARGO_PKG_NAME");
    tracing::info!("Starting {} version {}...", BIN_NAME, VERSION);

    run(args).await;

    tracing::info!("done, shutdown.");
}
