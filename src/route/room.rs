use std::sync::Arc;

use redis::Connection;
use tokio::sync::Mutex;
use warp::Filter;

use crate::controller::room_controller::{create_room, delete_room};

use super::with_redis;

pub fn create_room_handler(
    redis: Arc<Mutex<Connection>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("room")
        .and(warp::post())
        .and(with_redis(redis))
        .and(warp::body::json())
        .and_then(create_room)
}

pub fn delete_room_handler(
    redis: Arc<Mutex<Connection>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("room")
        .and(warp::delete())
        .and(with_redis(redis))
        .and(warp::path::param::<String>())
        .and_then(delete_room)
}
