use diesel::Insertable;
use serde::{Deserialize, Serialize};
use crate::models::client::CacheClientDGS;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DedicatedGameServer {
    pub id: uuid::Uuid,
    pub ip: std::net::IpAddr,
    pub port: u16,
    pub players: Vec<CacheClientDGS>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DedicatedGameServerRegister {
    pub ip: std::net::IpAddr,
    pub port: u16,
}

pub struct RatedDgs {
    pub dgs: DedicatedGameServer,
    pub rating: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DgsCluster {
    pub name: String,
    pub dgs: Vec<DedicatedGameServer>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::client)]
pub struct InsertableDGS<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub salt: &'a str,
    pub role_id: i32

}