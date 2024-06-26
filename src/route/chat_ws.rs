use crate::{controller::chat_controller::handle_connection, middleware};
use ::redis::Connection;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use warp::{filters::ws::Message, Filter};

use super::with_redis;

pub type Users = Arc<Mutex<Vec<mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;
type Handler = (warp::ws::Ws, Arc<Mutex<Connection>>, Users);

pub fn chat_websocket(
    redis: Arc<Mutex<Connection>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let users: Users = Arc::new(Mutex::new(Vec::new()));

    let chat_route = warp::path("chat")
        .and(warp::ws())
        .and(warp::header::optional("Authorization"))
        .and(with_redis(redis))
        .and(with_users(users))
        .and_then(middleware::authorization::authenticate)
        .map(|(ws, redis, users): Handler| {
            ws.on_upgrade(move |socket| handle_connection(socket, redis, users))
        });

    return chat_route;
}

fn with_users(
    users: Users,
) -> impl Filter<Extract = (Users,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || users.clone())
}
