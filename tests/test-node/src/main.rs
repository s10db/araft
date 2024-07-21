#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused)]

use clap::{Parser, Subcommand};

use araft::Node;
// use araft::NodeErr;
// use araft::{comm, interface};

#[derive(Parser, Debug)]
#[command(version)]
#[command(next_line_help = true)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(clap::Args, Debug)]
struct NewArgs {
    /// raft node id
    #[arg(short, long)]
    id: String,

    /// rpc bind address
    #[arg(short = 'a', long, default_value="[::1]")]
    rpc_bind_addr: String,

    /// rpc bind port
    #[arg(short = 'p', long)]
    rpc_bind_port: u16,

    /// list of peers to connect to, delimiter = ' '
    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    peers: Vec<String>,
}

#[derive(clap::Args, Debug)]
struct LoadArgs {
    /// path and file of the dump to load
    #[arg(short, long, value_name = "FILE")]
    dump_path_file: std::path::PathBuf,
}

#[derive(Subcommand, Debug)]
enum Mode {
    New(NewArgs),
    Load(LoadArgs),
}

async fn run_new(args: NewArgs) {
    let ncd = araft::NodeConfigData {
        raft_node_id: args.id,
        mods: araft::ModConfigData { 
            comm: araft::CommConfig {
                peers: args.peers,
            },
            interface: araft::InterfaceConfig {
                cli_addr: format!("{}:{}", args.rpc_bind_addr, args.rpc_bind_port).parse().unwrap()
            },
            state: araft::StateConfig::default()
        },
    };
    let node = Node::new(ncd);

    node.run().await;
}

async fn run_load(args: LoadArgs) {
    let node = Node::load(args.dump_path_file);
    node.run().await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const BIN_NAME: &'static str = env!("CARGO_PKG_NAME");
    tracing::info!("starting {BIN_NAME} version {VERSION}...");

    match args.mode {
        Mode::New(a) => run_new(a).await,
        Mode::Load(l) => run_load(l).await,
    }
        
    tracing::info!("done, shutdown.");
}
