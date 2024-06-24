pub fn insert_message(con: &mut redis::Connection, user: &str, message: &str) {
    let _ : () = redis::cmd("SET")
        .arg(user)
        .arg(message)
        .query(con)
        .expect("Fail to insert new row");
}
