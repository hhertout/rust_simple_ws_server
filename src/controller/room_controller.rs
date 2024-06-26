use std::sync::Arc;

use redis::Connection;
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::reply::{json, Json};

pub fn create_room(_redis: Arc<Mutex<Connection>>) -> Json {
    let id = Uuid::new_v4();

    return json(&json!({"status": "ok"}));
}

pub fn delete_room(_redis: Arc<Mutex<Connection>>) -> Json {
    return json(&json!({"status": "ok"}));
}
