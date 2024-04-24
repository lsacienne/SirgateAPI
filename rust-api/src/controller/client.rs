use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
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

pub fn get_user_by_id(connection: &mut PgConnection, user_id: uuid::Uuid) -> Client {
    use crate::schema::client::dsl::*;

    client
        .filter(id.eq(user_id))
        .first(connection)
        .expect("Error loading user")
}

pub fn get_user_by_email(connection: &mut PgConnection, _email: &str) -> Client {
    use crate::schema::client::dsl::*;

    client
        .filter(email.eq(_email))
        .first(connection)
        .expect("Error loading user")
}

pub fn does_user_exist(connection: &mut PgConnection, user_id: uuid::Uuid) -> bool {
    use crate::schema::client::dsl::*;

    let result = client
        .filter(id.eq(user_id))
        .first::<Client>(connection);

    match result {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn is_user_dgs(connection: &mut PgConnection, user_id: uuid::Uuid) -> bool {
    use crate::schema::client;
    use crate::schema::role;

    let result = client::table
        .inner_join(role::table.on(client::role_id.eq(role::role_id)))
        .filter(
            client::id.eq(user_id).and(role::role_name.eq("DGS"))
        )
        .select((client::id, role::role_name))
        .first::<(uuid::Uuid, String)>(connection);
        

    match result {
        Ok(_) => true,
        Err(_) => false
    }
}