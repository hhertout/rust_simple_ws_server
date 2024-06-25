use dotenv::dotenv;

pub(crate) mod controller;
pub(crate) mod route;
pub(crate) mod database;
pub(crate) mod repository;
pub(crate) mod middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let route = route::chat_websocket();
    let port = std::env::var("PORT")
        .expect("Variable port is not defined")
        .parse::<u16>()
        .expect("Failed to parse port Variable");

    println!("Running server on port {}", port);
    warp::serve(route).run(([127, 0, 0, 1], port)).await
}
