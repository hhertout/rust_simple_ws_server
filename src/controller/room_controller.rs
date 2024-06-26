use std::sync::Arc;

use redis::Connection;
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::{
    reject::{self, Rejection},
    reply::{json, Json},
};

use crate::repository;

use super::CustomError;

pub(crate) async fn create_room(redis: Arc<Mutex<Connection>>) -> Result<Json, Rejection> {
    let id = Uuid::new_v4();
    let mut con = redis.lock().await;
    let token = "azertyuiop";
    if let Err(err) = repository::room::insert_new_room(&mut con, id, token) {
        return Err(reject::custom(CustomError{message: err.to_string()}))
    };

    return Ok(json(
        &json!({"room": id.to_string().to_owned() ,"token": token.to_owned()}),
    ));
}

pub(crate) async fn delete_room(_redis: Arc<Mutex<Connection>>) -> Result<Json, Rejection> {
    return Ok(json(&json!({"status": "ok"})));
}
