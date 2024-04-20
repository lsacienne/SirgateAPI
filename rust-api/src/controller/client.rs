use diesel::{query_dsl::methods::{LimitDsl, FilterDsl}, ExpressionMethods, PgConnection, RunQueryDsl};
use crate::models::client::{InsertableClient, Client};

pub fn add_user<'a>(
    connection: &mut PgConnection,
    username: &'a str,
    email: &'a str,
    password_hash: &'a str,
    salt: &'a str,
) -> Client {
    use crate::schema::client;

    let new_user = InsertableClient {
        username,
        email,
        password: password_hash,
        salt
    };

    diesel::insert_into(client::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user")
}

pub fn show_users(connection: &mut PgConnection) -> Vec<Client> {
    use crate::schema::client::dsl::*;

    client
        .load::<Client>(connection)
        .expect("Error loading users")
}

pub fn show_limited_users(connection: &mut PgConnection, limit: i64) -> Vec<Client> {
    use crate::schema::client::dsl::*;

    client
        .limit(limit)
        .load::<Client>(connection)
        .expect("Error loading users")
}

pub fn get_user_by_username(connection: &mut PgConnection, user_name: &str) -> Client {
    use crate::schema::client::dsl::*;

    client
        .filter(username.eq(user_name))
        .first(connection)
        .expect("Error loading user")
}

pub fn get_user_by_email(connection: &mut PgConnection, email: &str) -> Client {
    use crate::schema::client::dsl::*;

    client
        .filter(email.eq(email))
        .first(connection)
        .expect("Error loading user")
}