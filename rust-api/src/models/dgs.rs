use serde::{Deserialize, Serialize};
use crate::models::client::CacheClient;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DedicatedGameServer {
    pub id: uuid::Uuid,
    pub ip: std::net::IpAddr,
    pub port: u16,
    pub players: Vec<CacheClient>,
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