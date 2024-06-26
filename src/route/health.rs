use serde_json::json;
use warp::Filter;

pub fn health_route(
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&json!({"status": "ok"})))
}
