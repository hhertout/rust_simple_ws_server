use std::sync::Arc;

use tokio::sync::Mutex;

use crate::route::chat_ws::Users;

#[derive(Debug)]
struct InvalidToken;

impl warp::reject::Reject for InvalidToken {}

#[derive(Debug)]
struct MissingToken;

impl warp::reject::Reject for MissingToken {}

pub async fn authenticate(
    ws: warp::ws::Ws,
    token: Option<String>,
    redis: Arc<Mutex<redis::Connection>>,
    users: Users,
) -> Result<(warp::ws::Ws, Arc<Mutex<redis::Connection>>, Users), warp::Rejection> {
    if let Some(t) = token {
        if t == "toto" {
            Ok((ws, redis, users))
        } else {
            Err(warp::reject::custom(InvalidToken))
        }
    } else {
        Err(warp::reject::custom(MissingToken))
    }
}