use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use rust_api::controller::dgs::add_dgs;
use rust_api::models::client::Client;
use rust_api::view::client;

#[tokio::main]
async fn main() {
    let mut con = establish_connection();

    register_dgs(&mut con, "server1", "server1", "s1@servers.com").await.unwrap();
    register_dgs(&mut con, "server2", "server2", "s2@servers.com").await.unwrap();
    register_dgs(&mut con, "server3", "server3", "s3@servers.com").await.unwrap();
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn register_dgs(con: &mut PgConnection, username: &str, password: &str, email: &str ) -> Result<Client, Box<dyn std::error::Error>> {
    let salt = client::generate_salt();
    let hash = client::hash_password(&password, &salt).unwrap();

    let dgs_client = add_dgs(&mut *con, username, email, &*hash.hash.unwrap().to_string(), salt.as_ref())?;

    Ok(dgs_client)
}