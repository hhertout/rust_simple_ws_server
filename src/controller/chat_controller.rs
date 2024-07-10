use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::{sync::mpsc, time::interval};
use warp::filters::ws::{Message, WebSocket};

use crate::route::chat_ws::Users;

#[derive(Debug, Serialize, Deserialize)]
struct ChatEvent {
    user: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DisconnectEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateEvent {
    user_connected: usize,
}
#[derive(Debug, Serialize, Deserialize)]
struct ErrorEvent {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "event")]
enum SocketEvent {
    #[serde(rename = "chat")]
    ChatEvent(ChatEvent),
    #[serde(rename = "update")]
    UpdateEvent(UpdateEvent),
    #[serde(rename = "disconnect")]
    DisconnectEvent(DisconnectEvent),
    #[serde(rename = "error")]
    ErrorEvent(ErrorEvent),
}

pub(crate) async fn handle_connection(ws: WebSocket, users: Users) {
    let (mut user_sender, mut user_receiver) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();

    let mut rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    let user_id: usize;

    {
        let mut users_guard = users.lock().await;
        user_id = users_guard.len();
        users_guard.push((user_id, tx));
    }

    let user_clone = Arc::clone(&users);
    let user_clone_bis = Arc::clone(&users);

    tokio::spawn(async move {
        while let Some(result) = rx.next().await {
            if let Ok(msg) = result {
                if user_sender.send(msg).await.is_err() {
                    println!("Connexion lost...");
                    break;
                }
            }
        }

        let mut users_guard = user_clone.lock().await;
        if let Some(pos) = users_guard.iter().position(|(id, _)| *id == user_id) {
            users_guard.remove(pos);
            println!("A user disconnect...");
        }
    });

    let mut interval = interval(Duration::from_secs(30));
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
                println!("A user will be disconnected");
                users_guard.remove(index);

                if users_guard.len() > 0 {
                    let event = DisconnectEvent {
                        user: None,
                        message: Some(String::from("A user was disconnecting from the room")),
                    };
                    
                    for (_, tx) in users_guard.iter() {
                        let to_send = serde_json::to_string(&event);
                        match to_send {
                            Ok(message) => {
                                let _ = tx.send(Ok(Message::text(message)));
                            }
                            Err(err) => {
                                println!("Failed to parse disconnect message {}", err);
                            }
                        }
                    }
                };
                return;
            }

            if users_guard.len() > 0 {
                let user_con = UpdateEvent {
                    user_connected: users_guard.len(),
                };
                let event = SocketEvent::UpdateEvent(user_con);

                for (_, tx) in users_guard.iter() {
                    match serde_json::to_string(&event) {
                        Ok(message) => {
                            let _ = tx.send(Ok(Message::text(message)));
                        }
                        Err(err) => {
                            println!("Failed to parse the update message {}", err);
                        }
                    };
                }
            }
        }
    });

    while let Some(result) = user_receiver.next().await {
        match result {
            Ok(msg) => handle_message(msg, &users, user_id).await,
            Err(_) => {
                println!("Connection closed");
                break;
            }
        }
    }
}

async fn handle_message(msg: Message, users: &Users, user_id: usize) {
    let text = match msg.to_str() {
        Ok(text) => text,
        Err(err) => return println!("Error : Message cannot be parsed : {:?}", err),
    };

    match serde_json::from_str::<SocketEvent>(text) {
        Ok(socket_event) => {
            let response_message = match socket_event {
                SocketEvent::ChatEvent(event) => {
                    let event_to_send = ChatEvent {
                        user: event.user.clone(),
                        message: event.message.clone(),
                    };
                    SocketEvent::ChatEvent(event_to_send)
                }
                SocketEvent::DisconnectEvent(event) => {
                    let user = match event.user {
                        Some(user) => user,
                        None => return,
                    };

                    let mut user_gard = users.lock().await;
                    user_gard.remove(user_id);

                    let event_to_send = DisconnectEvent {
                        user: None,
                        message: Some(format!("User {} left the room", user)),
                    };

                    SocketEvent::DisconnectEvent(event_to_send)
                }
                _ => return,
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
                Err(err) => {
                    let err_message = SocketEvent::ErrorEvent(ErrorEvent {
                        message: String::from(format!("Failed to send message to user: {}", err)),
                    });
                    let to_send = serde_json::to_string(&err_message).unwrap();
                    for (_, tx) in users.lock().await.iter() {
                        if let Err(e) = tx.send(Ok(Message::text(to_send.clone()))) {
                            println!("Failed to send message to user: {}", e);
                        }
                    }
                    return println!("Error : Message cannot be parsed : {}", err);
                }
            };
        }
        Err(err) => {
            let err_message = SocketEvent::ErrorEvent(ErrorEvent {
                message: String::from(format!("Failed to send message to user: {}", err)),
            });
            let to_send = serde_json::to_string(&err_message).unwrap();
            for (_, tx) in users.lock().await.iter() {
                if let Err(e) = tx.send(Ok(Message::text(to_send.clone()))) {
                    println!("Failed to send message to user: {}", e);
                }
            }
            return println!("Error : Message cannot be parsed : {}", err);
        }
    }
}
