#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused)]

macro_rules! one_recv {
    // case for empty ActorMessage
    ($cmd:ident, $chan:expr) => {{
        let (resp_tx, resp_rx) = oneshot::channel();
        let msg = ActorMessage::$cmd {
            resp: resp_tx
        };
        $chan.send(msg).await.expect("could not send to channel");
        resp_rx.await
    }};
    // case for ActorMessage with members
    ($cmd:ident, $chan:expr, $($data_name:ident, $data_val:expr),*) => {{
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        let msg = ActorMessage::$cmd {
            $($data_name: $data_val),*,
            resp: resp_tx
        };
        $chan.send(msg).await.expect("could not send to channel");
        resp_rx.await.expect("could not send to channel")
    }};
}

mod actor;
mod comm;
mod config;
mod core;
mod interface;
mod log;
mod metal;
mod state;

pub struct Node;
pub struct Client;