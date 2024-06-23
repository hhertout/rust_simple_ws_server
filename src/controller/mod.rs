use std::sync::{Arc, Mutex};

use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::filters::ws::{Message, WebSocket};

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    user: String,
    message: String,
}

type Users = Arc<Mutex<Vec<mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

pub(crate) async fn handle_connection(ws: WebSocket) {
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();

    let rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    let users = Arc::new(Mutex::new(Vec::new()));
    users.lock().unwrap().push(tx);

    let user_clone = Arc::clone(&users);

    tokio::task::spawn(async move {
        let mut rx = rx;
        while let Some(result) = rx.next().await {
            if let Ok(msg) = result {
                user_ws_tx.send(msg).await.unwrap();
            }
        }
    });

    while let Some(result) = user_ws_rx.next().await {
        if let Ok(msg) = result {
            handle_message(msg, &user_clone).await
        }
    }
}

async fn handle_message(msg: Message, users: &Users) {
    if let Ok(text) = msg.to_str() {
        if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(text) {
            println!("Received a new message : {:?}", chat_msg);

            let response_message = ChatMessage {
                user: String::from("bot"),
                message: String::from("Message successfully received"),
            };

            let message = serde_json::to_string(&response_message).unwrap();
            let to_send = Message::text(message);

            for tx in users.lock().unwrap().iter() {
                tx.send(Ok(to_send.clone())).unwrap();
            }
        }
    }
}
