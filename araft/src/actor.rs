use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<T>;

pub struct Actor<T, D> {
    pub data: D,
    receiver: mpsc::Receiver<T>,
}

impl<T, D> Actor<T, D> {
    fn new(receiver: mpsc::Receiver<T>, data: D) -> Self {
        Self { 
            receiver,
            data
        }
    }
}

#[derive(Clone)]
pub struct ActorHandle<T> {
    sender: mpsc::Sender<T>
}

impl<T> ActorHandle<T>
where
    T : std::marker::Send + 'static
{
    pub async fn new<F, Fut, D>(buffer_size: usize, mut handle_actor_msg_fn: F, data: D) -> Self
    where 
        F: FnMut(&T, &mut D) -> Fut + std::marker::Send + 'static,
        Fut: std::future::Future<Output=bool>,
        D : std::marker::Send + 'static
    {
        let (sender, receiver) = mpsc::channel(buffer_size);
        let actor = Actor::new(receiver, data);
        
        //tokio::spawn(async move {run_actor::<T, D, F, Fut>(actor, handle_actor_msg_fn).await});
        
        Self {
            sender
        }
    }
}

async fn run_actor<T, D, F, Fut>(mut actor: Actor<T, D>, mut handle_actor_msg_fn: F) 
where 
    F: FnMut(&T, &mut D) -> Fut,
    Fut: std::future::Future<Output=bool>
{
    tracing::info!("Starting comm actor");
    while let Some(msg) = actor.receiver.recv().await {
        if !handle_actor_msg_fn(&msg, &mut actor.data).await {
            break;
        }
    }
    tracing::info!("Closing comm actor");
}