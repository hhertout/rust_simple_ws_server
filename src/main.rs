use dotenv::dotenv;
use warp::Filter;

pub(crate) mod controller;
pub(crate) mod database;
pub(crate) mod middleware;
pub(crate) mod repository;
pub(crate) mod route;

#[tokio::main]
async fn main() {
    dotenv().ok();

    if std::env::var("RUST_ENV").unwrap_or_default() == String::from("development") {
        println!("🛠️ Server is running under development mode")
    }

    let port = std::env::var("PORT")
        .expect("❌ Variable port is not defined")
        .parse::<u16>()
        .expect("❌ Failed to parse port Variable");

    let incoming_log = warp::log::custom(|info| {
        eprintln!(
            "[{}] {} {} {} {:?}",
            chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S"),
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
        );
    });

    let route = crate::route::routes().with(incoming_log);

    println!("🚀 Server is running on port {}", port);
    warp::serve(route).run(([127, 0, 0, 1], port)).await
}
