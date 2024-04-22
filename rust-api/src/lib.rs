use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use diesel::r2d2::ConnectionManager;

pub mod view{
    pub mod client;
    pub mod achievement;
    pub mod ranking;
}
pub mod models{
    pub mod client;
    pub mod achievement;
    pub mod friends;
    pub mod dgs;
    pub mod ranking;
}
pub mod controller{
    pub mod achievement;
    pub mod database_manager;
    pub mod dgs;
    pub mod client;
    pub mod ranking;
    pub mod friends;
}

mod schema;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;