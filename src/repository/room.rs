use redis::{Commands, Connection, RedisError};
use uuid::Uuid;

pub fn insert_new_room(con: &mut Connection, uuid: Uuid, users: Vec<String>) -> Result<(), RedisError> {
    let json_users = serde_json::to_string(&users).unwrap();
    redis::cmd("SET").arg(uuid.to_string()).arg(json_users).query(con)
}

pub fn get_room_by_id(con: &mut Connection, uuid: String) -> Result<String, RedisError> {
    con.get(&uuid)
}

pub fn delete_room(con: &mut Connection, uuid: String) -> Result<(), RedisError>{
    con.del(&uuid)
}

pub fn get_room_count(con: &mut Connection) -> Result<usize, RedisError> {
    redis::cmd("DBSIZE").query(con)
}