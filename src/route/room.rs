use std::sync::Arc;

use redis::Connection;
use tokio::sync::Mutex;
use warp::Filter;

use crate::controller;

use super::with_redis;

pub fn create_room(
    redis: Arc<Mutex<Connection>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("room")
        .and(warp::post())
        .and(with_redis(redis))
        .map(controller::room_controller::create_room)
}

pub fn delete_room(
    redis: Arc<Mutex<Connection>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("room")
        .and(warp::delete())
        .and(with_redis(redis))
        .map(controller::room_controller::delete_room)
}
