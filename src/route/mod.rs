use crate::{controller, database::redis, middleware};
use ::redis::Connection;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex as AsyncMutex};
use warp::{filters::ws::Message, Filter};

pub type Users = Arc<AsyncMutex<Vec<mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;
type Handler = (warp::ws::Ws, Arc<AsyncMutex<Connection>>, Users);

pub fn chat_websocket(
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let redis_client = redis::connect();
    let redis = Arc::new(AsyncMutex::new(redis_client));

    let users: Users = Arc::new(AsyncMutex::new(Vec::new()));

    let chat_route = warp::path("chat")
        .and(warp::ws())
        .and(warp::header::optional("Authorization"))
        .and(with_redis(redis))
        .and(with_users(users))
        .and_then(middleware::authorization::authenticate)
        .map(|(ws, redis, users): Handler| {
            ws.on_upgrade(move |socket| controller::handle_connection(socket, redis, users))
        });

    return chat_route;
}

fn with_redis(
    redis: Arc<AsyncMutex<Connection>>,
) -> impl Filter<Extract = (Arc<AsyncMutex<Connection>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || Arc::clone(&redis))
}

fn with_users(
    users: Users,
) -> impl Filter<Extract = (Users,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || users.clone())
}
