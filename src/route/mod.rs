use warp::Filter;

use crate::controller;

pub fn router() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let chat = warp::path("chat")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(controller::handle_connection));

    return chat;
}
