use std::sync::Arc;

use health::health_route;
use redis::Connection;
use room::{create_room, delete_room};
use tokio::sync::{mpsc, Mutex};
use warp::{filters::ws::Message, Filter};

use crate::database::redis::connect;

pub(crate) mod chat_ws;
pub(crate) mod health;
pub(crate) mod room;

pub fn routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let redis_client = connect();
    let redis = Arc::new(Mutex::new(redis_client));

    let route = crate::chat_ws::chat_websocket(Arc::clone(&redis))
        .or(health_route())
        .or(create_room(Arc::clone(&redis)))
        .or(delete_room(Arc::clone(&redis)));

    return route;
}

fn with_redis(
    redis: Arc<Mutex<Connection>>,
) -> impl Filter<Extract = (Arc<Mutex<Connection>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&redis))
}
