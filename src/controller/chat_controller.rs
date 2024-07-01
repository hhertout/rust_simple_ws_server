use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::{sync::mpsc, time::interval};
use warp::filters::ws::{Message, WebSocket};

use crate::route::chat_ws::Users;

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    user: String,
    message: String,
}

pub(crate) async fn handle_connection(ws: WebSocket, users: Users) {
    println!("A new user join the room !");

    let (mut user_sender, mut user_receiver) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();

    let mut rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    let user_id: usize;

    {
        let mut users_guard = users.lock().await;
        user_id = users_guard.len();
        users_guard.push((user_id, tx));
        println!("Entering the room !");
    }

    let user_clone = Arc::clone(&users);
    let user_clone_bis = Arc::clone(&users);

    let mut interval = interval(Duration::from_secs(10));

    tokio::spawn(async move {
        while let Some(result) = rx.next().await {
            if let Ok(msg) = result {
                if user_sender.send(msg).await.is_err() {
                    println!("Connexion lost...");
                    break;
                }
            }
        }

        // Suppression de l'expéditeur de la liste des utilisateurs lors de la déconnexion
        let mut users_guard = user_clone.lock().await;
        if let Some(pos) = users_guard.iter().position(|(id, _)| *id == user_id) {
            users_guard.remove(pos);
            println!("A user disconnect...");
        }
    });

    tokio::spawn(async move {
        loop {
            interval.tick().await;

            let mut users_guard = user_clone_bis.lock().await;
            let mut disconnected_users = Vec::new();

            for (index, (_, tx)) in users_guard.iter().enumerate() {
                if tx.is_closed() {
                    disconnected_users.push(index);
                }
            }

            for index in disconnected_users.into_iter().rev() {
                users_guard.remove(index);
            }

            if users_guard.len() > 0 {
                for (_, tx) in users_guard.iter() {
                    let message = format!("User connected: {}", users_guard.len());
                    let _ = tx.send(Ok(Message::text(message)));
                }
            }
        }
    });

    while let Some(result) = user_receiver.next().await {
        match result {
            Ok(msg) => handle_message(msg, &users).await,
            Err(_) => {
                println!("Connection closed");
                break;
            }
        }
    }
}

async fn handle_message(msg: Message, users: &Users) {
    let text = match msg.to_str() {
        Ok(text) => text,
        Err(_) => return println!("Error : Message cannot be parsed"),
    };

    match serde_json::from_str::<ChatMessage>(text) {
        Ok(chat_msg) => {
            println!(
                "[New message] : {} sent : '{}'",
                chat_msg.user, chat_msg.message
            );

            let response_message = ChatMessage {
                user: chat_msg.user.clone(),
                message: chat_msg.message.clone(),
            };

            match serde_json::to_string(&response_message) {
                Ok(message) => {
                    let to_send = Message::text(message);
                    for (_, tx) in users.lock().await.iter() {
                        if let Err(e) = tx.send(Ok(to_send.clone())) {
                            println!("Failed to send message to user: {:?}", e);
                        }
                    }
                }
                Err(_) => return println!("Error : Message cannot be parsed"),
            };
        }
        Err(_) => return println!("Error : Message cannot be parsed"),
    }
}
