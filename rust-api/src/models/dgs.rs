use serde::{Deserialize, Serialize};
use crate::models::client::CacheClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedicatedGameServer<'a> {
    pub name: String,
    pub ip: std::net::IpAddr,
    pub port: u16,
    pub players: Vec<CacheClient<'a>>,
}