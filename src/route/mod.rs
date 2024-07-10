use std::sync::Arc;

use chat_ws::chat_websocket;
use health::health_route;
use redis::Connection;
use room::{create_room_handler, delete_room_handler, get_room_count_handler};
use tokio::sync::Mutex;
use warp::Filter;

use crate::database::redis::connect;

pub(crate) mod chat_ws;
pub(crate) mod health;
pub(crate) mod room;

pub fn v1(
    redis: Arc<Mutex<Connection>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(create_room_handler(Arc::clone(&redis))
        .or(delete_room_handler(Arc::clone(&redis)))
        .or(get_room_count_handler(Arc::clone(&redis))))
}

pub fn routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let redis_client = connect();
    let redis = Arc::new(Mutex::new(redis_client));

    health_route()
        .or(chat_websocket(Arc::clone(&redis)))
        .or(v1(Arc::clone(&redis)))
}

fn with_redis(
    redis: Arc<Mutex<Connection>>,
) -> impl Filter<Extract = (Arc<Mutex<Connection>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&redis))
}
