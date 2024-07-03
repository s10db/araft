use crate::actor::{ActorHandle, Actor};

#[derive(Clone)]
pub enum ActorMessage {
}

struct Data {
}

#[derive(Clone)]
pub struct Comm {
    //pub actor: ActorHandle<ActorMessage>,
}

impl Comm {
    pub async fn new(actor_buffer_size: usize) -> Self {
        Self { 
            //actor: ActorHandle::new(actor_buffer_size, |msg, act|handle_msg(*msg, act), Data {}).await
        }
    }
}

async fn handle_msg(msg: ActorMessage, act: &mut Data) -> bool {
    false
}
































// use tokio::sync::{mpsc, oneshot};

// mod internal {
//     tonic::include_proto!("internal");
// }

// use internal::{
//     AppendEntriesIn,
//     AppendEntriesOut,
//     RequestVoteIn,
//     RequestVoteOut,
//     EchoIn,
//     EchoOut};

// type Responder<T> = oneshot::Sender<T>;

// enum ActorMessage {
// }

// struct CommActor {
//     receiver: mpsc::Receiver<ActorMessage>,
// }

// impl CommActor {
//     fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
//         Self { 
//             receiver
//         }
//     }
// }

// #[derive(Clone)]
// pub struct CommActorHandler {
//     sender: mpsc::Sender<ActorMessage>
// }

// impl CommActorHandler {
//     pub async fn new(buffer_size: usize) -> Self {
//         let (sender, receiver) = mpsc::channel(buffer_size);
//         let actor = CommActor::new(receiver);
        
//         tokio::spawn(async move {run_comm_actor(actor).await});
        
//         Self {
//             sender
//         }
//     }
// }

// async fn handle_actor_msg(msg: ActorMessage, actor: &mut CommActor) -> bool {
//     false
// }

// async fn run_comm_actor(mut actor: CommActor) {
//     tracing::info!("Starting comm actor");
//     while let Some(msg) = actor.receiver.recv().await {
//         if !handle_actor_msg(msg, &mut actor).await {
//             break;
//         }
//     }
//     tracing::info!("Closing comm actor");
// }

// #[derive(Clone)]
// pub struct Comm {
//     pub actor: CommActorHandler,
// }

// impl Comm {
//     pub async fn new(actor_buffer_size: usize) -> Self {
//         Self { 
//             actor: CommActorHandler::new(actor_buffer_size).await
//         }
//     }
// }