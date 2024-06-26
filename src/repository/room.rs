use redis::{Connection, RedisError};
use uuid::Uuid;

pub fn insert_new_room(con: &mut Connection, uuid: Uuid, token: &str) -> Result<(), RedisError> {
    redis::cmd("SET").arg(uuid.to_string()).arg(token).query(con)
}
