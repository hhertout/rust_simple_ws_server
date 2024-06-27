use crate::{controller::chat_controller::handle_connection, middleware};
use ::redis::Connection;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use warp::{filters::ws::Message, Filter};

use super::with_redis;

pub type UserTx = mpsc::UnboundedSender<Result<Message, warp::Error>>;
pub type Users = Arc<Mutex<Vec<(usize, UserTx)>>>;
type Handler = (warp::ws::Ws, Users);

pub fn chat_websocket(
    redis: Arc<Mutex<Connection>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let users: Users = Arc::new(Mutex::new(Vec::new()));

    warp::path("chat")
        .and(warp::ws())
        .and(warp::header::optional("Authorization"))
        .and(with_redis(redis))
        .and(with_users(users))
        .and(warp::path::param::<String>())
        .and_then(middleware::authorization::is_authenticate)
        .map(|(ws, users): Handler| ws.on_upgrade(move |socket| handle_connection(socket, users)))
}

fn with_users(
    users: Users,
) -> impl Filter<Extract = (Users,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || users.clone())
}
