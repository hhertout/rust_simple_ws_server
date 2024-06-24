pub fn connect() -> redis::Connection {
    let url = std::env::var("REDIS_URL").expect("REDIS_URL variable is not set");

    let conn = redis::Client::open(url)
    .expect("Failed to connect to the database, please check your url you provide or your credentials")
    .get_connection()
    .expect("Failed to connect to Redis");

    println!("[OK] Connection with redis etablished");
    return conn;
}