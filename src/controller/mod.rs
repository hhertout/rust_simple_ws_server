use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex as AsyncMutex;
use warp::filters::ws::{Message, WebSocket};

use crate::repository;
use crate::route::Users;

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    user: String,
    message: String,
}

#[derive(Clone)]
struct Context {
    redis: Arc<AsyncMutex<redis::Connection>>,
}

pub(crate) async fn handle_connection(
    ws: WebSocket,
    redis: Arc<AsyncMutex<redis::Connection>>,
    users: Users,
) {
    println!("A new user join !");
    let ctx = Context { redis };

    let (mut user_sender, mut user_receiver) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();

    let rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    {
        let mut users_guard = users.lock().await;
        users_guard.push(tx);
    }

    let user_clone = Arc::clone(&users);

    tokio::task::spawn(async move {
        let mut rx = rx;
        while let Some(result) = rx.next().await {
            if let Ok(msg) = result {
                user_sender.send(msg).await.unwrap();
            }
        }
    });

    while let Some(result) = user_receiver.next().await {
        if let Ok(msg) = result {
            handle_message(ctx.clone(), msg, &user_clone).await
        }
    }
}

async fn handle_message(ctx: Context, msg: Message, users: &Users) {
    if let Ok(text) = msg.to_str() {
        if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(text) {
            println!("[New message] : {} send : '{}'", chat_msg.user, chat_msg.message);

            let mut conn = ctx.redis.lock().await;

            repository::message::insert_message(
                &mut conn,
                chat_msg.user.clone().as_str(),
                chat_msg.message.clone().as_str(),
            );
            let response_message = ChatMessage {
                user: chat_msg.user,
                message: chat_msg.message,
            };

            let message = serde_json::to_string(&response_message).unwrap();
            let to_send = Message::text(message);

            for tx in users.lock().await.iter() {
                tx.send(Ok(to_send.clone())).unwrap();
            }
        }
    }
}
