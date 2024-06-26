use std::sync::Arc;

use redis::Connection;
use serde_json::json;
use tokio::sync::Mutex;
use warp::reply::{json, Json};

pub fn create_room(redis: Arc<Mutex<Connection>>) -> Json {
    return json(&json!({"status": "ok"}));
}

pub fn delete_room(redis: Arc<Mutex<Connection>>) -> Json {
    return json(&json!({"status": "ok"}));
}
