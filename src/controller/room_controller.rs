use std::sync::Arc;

use redis::Connection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::{
    reject::{self, Rejection},
    reply::{json, Json},
};

use crate::repository;

use super::CustomError;

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateRoomBody {
    pub users: Vec<String>,
}

pub(crate) async fn create_room(
    redis: Arc<Mutex<Connection>>,
    body: CreateRoomBody,
) -> Result<Json, Rejection> {
    if body.users.len() != 2 {
        return Err(reject::custom(CustomError {
            message: String::from("Only two users are authorized to join a room"),
        }));
    }
    let id = Uuid::new_v4();
    let mut con = redis.lock().await;
    if let Err(err) = repository::room::insert_new_room(&mut con, id, body.users.clone()) {
        return Err(reject::custom(CustomError {
            message: err.to_string(),
        }));
    };

    return Ok(json(
        &json!({"room": id.to_string().to_owned() ,"users": &json!(body.users)}),
    ));
}

pub(crate) async fn delete_room(
    redis: Arc<Mutex<Connection>>,
    uuid: String,
) -> Result<Json, Rejection> {
    let mut con = redis.lock().await;
    match repository::room::delete_room(&mut con, uuid) {
        Ok(_) => Ok(json(&json!({"status": "ok"}))),
        Err(err) => Err(reject::custom(CustomError {
            message: err.to_string(),
        })),
    }
}

pub(crate) async fn get_room_count(redis: Arc<Mutex<Connection>>) -> Result<Json, Rejection> {
    let mut con = redis.lock().await;
    match repository::room::get_room_count(&mut con) {
        Ok(count) => Ok(json(&json!({"count": count}))),
        Err(err) => Err(reject::custom(CustomError {
            message: err.to_string(),
        })),
    }
}
