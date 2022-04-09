use futures::StreamExt;
use log::info;
use std::{collections::HashMap, convert::Infallible, env, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

static NEXT_USERID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

// k - id
// v - Message sender
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

#[tokio::main]
async fn main() {
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let users = Users::default();
    let chat = warp::path("ws")
        .and(warp::ws())
        .and(with_users(users))
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| connect(socket, users)));

    let files = warp::fs::dir("static");
    let routes = chat.or(files);
    warp::serve(routes).run(([127, 0, 0, 1], 7070)).await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

async fn connect(ws: WebSocket, users: Users) {
    let my_id = NEXT_USERID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    info!("Connect UserId {}", my_id);

    let (user_tx, mut user_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();

    let rx = UnboundedReceiverStream::new(rx);
    tokio::spawn(rx.forward(user_tx));
    users.write().await.insert(my_id, tx);

    while let Some(result) = user_rx.next().await {
        broadcast(result.unwrap(), &users).await
    }

    disconnect(my_id, &users);
}

async fn broadcast(msg: Message, users: &Users) {
    if let Ok(_) = msg.to_str() {
        for (&uid, tx) in users.read().await.iter() {
            println!("uid: {}, msg: {:?}", uid, msg.clone());
            tx.send(Ok(msg.clone())).expect("Failed to send message");
        }
    }
}

async fn disconnect(my_id: usize, users: &Users) {
    info!("disconnect id: {}", my_id);
    users.write().await.remove(&my_id);
}
