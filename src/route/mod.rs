use crate::{controller, database::redis};
use ::redis::Connection;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, Mutex as AsyncMutex};
use warp::{filters::ws::Message, Filter};

pub type Users = Arc<Mutex<Vec<mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

pub fn router() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let redis_client = redis::connect();
    let redis = Arc::new(AsyncMutex::new(redis_client));

    let users: Users = Arc::new(Mutex::new(Vec::new()));

    let chat_route = warp::path("chat")
        .and(warp::ws())
        .and(with_redis(redis))
        .and(with_users(users))
        .map(|ws: warp::ws::Ws, redis, users| {
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
