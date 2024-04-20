use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use redis::RedisResult;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_redis_connection() -> RedisResult<redis::Connection> {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL")
        .expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url)
        .unwrap();
    client.get_connection()
}