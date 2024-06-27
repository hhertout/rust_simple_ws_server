use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{controller::CustomError, repository, route::chat_ws::Users};

#[derive(Debug)]
pub struct MissingToken {
    pub message: String,
}

impl warp::reject::Reject for MissingToken {}

pub async fn is_authenticate(
    ws: warp::ws::Ws,
    token: Option<String>,
    redis: Arc<Mutex<redis::Connection>>,
    users: Users,
    room_id: String,
) -> Result<(warp::ws::Ws, Users), warp::Rejection> {
    if let None = token {
        return Err(warp::reject::custom(MissingToken {
            message: String::from("Token is missing"),
        }));
    }
    // Check the token from the confidence authority
    // get back the @user_id or email
    let user_id = String::from("toto@gmail.com");

    let db = Arc::clone(&redis);
    let mut con = db.lock().await;

    let response = match repository::room::get_room_by_id(&mut con, room_id) {
        Ok(room_data) => room_data,
        Err(_) => {
            println!("Authorization middleware - Room is not found");
            return Err(warp::reject::custom(CustomError {
                message: String::from("Room is not found"),
            }));
        }
    };

    let room_users: Vec<String> = match serde_json::from_str(&response) {
        Ok(json) => json,
        Err(err) => {
            println!("Authorization middleware - Fail to parse the query result");
            return Err(warp::reject::custom(CustomError {
                message: err.to_string(),
            }));
        }
    };

    if !room_users.contains(&user_id) {
        return Err(warp::reject::custom(CustomError {
            message: String::from("Unauthorized"),
        }));
    }

    Ok((ws, users))
}
