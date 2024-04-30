use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use diesel::result::Error;
use redis::{cmd, JsonCommands};
use crate::models::client::{CacheClient, CacheClientDGS, Client, ClientState, InsertableClient};
use crate::unescape;

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
pub fn get_user_by_username_non_blocking(connection: &mut PgConnection, user_name: &str) -> Result<Client,Error> {
    use crate::schema::client::dsl::*;
    client
        .filter(username.eq(user_name))
        .first(connection)
}

pub fn get_user_by_id(connection: &mut PgConnection, user_id: uuid::Uuid) -> Client {
    use crate::schema::client::dsl::*;

    client
        .filter(id.eq(user_id))
        .first(connection)
        .expect("Error loading user")
}

pub fn get_limited_user_by_id(connection: &mut PgConnection, user_id: uuid::Uuid) -> CacheClient {
    use crate::schema::client;
    use crate::schema::rank;

    let user_client: (uuid::Uuid, String, String) = client::table
        .inner_join(rank::table.on(client::rank_id.eq(rank::id)))
        .filter(client::id.eq(user_id))
        .select((client::id, client::username, rank::rank_name))
        .first(connection)
        .expect("Error loading user");
     CacheClient {
        id: user_client.0,
        username: user_client.1,
        rank: user_client.2,
        state: ClientState::Online
    }
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

pub fn initialize_client_cache(connection: &mut redis::Connection) -> () {
    let client_array: Vec<CacheClientDGS> = vec![];
    connection.json_set::<_, _, Vec<CacheClientDGS>, ()>("ALL_CLIENTS", "$", &client_array).unwrap()
}

pub fn cache_client(connection: &mut PgConnection, mut redis_connection: redis::Connection, user_id: uuid::Uuid) -> CacheClient {
    use crate::schema::client;
    use crate::schema::rank;

    let user_client: (uuid::Uuid, String, String) = client::table
        .inner_join(rank::table.on(client::rank_id.eq(rank::id)))
        .filter(client::id.eq(user_id))
        .select((client::id, client::username, rank::rank_name))
        .first(connection)
        .expect("Error loading user");

    let client_cache = CacheClient {
        id: user_client.0,
        username: user_client.1,
        rank: user_client.2,
        state: ClientState::Online
    };
    redis_connection.json_arr_append::<_, _, _, ()>("ALL_CLIENTS", "$", &client_cache).unwrap();
    client_cache
}

pub fn is_client_cached(mut redis_connection: redis::Connection, user_id: uuid::Uuid) -> bool {
    let client_list_string = redis_connection.json_get::<_, _, String>("ALL_CLIENTS","$").unwrap();
    let client_list = serde_json::from_str::<Vec<Vec<CacheClient>>>(&client_list_string).unwrap();
    let client_list =client_list.get(0).unwrap();

    match client_list.iter().position(|client| client.id == user_id) {
        Some(_) => true,
        None => false
    }
}

pub fn get_cached_client(mut redis_connection: redis::Connection, user_id: uuid::Uuid) -> Option<CacheClient> {
    let client_list_string = redis_connection.json_get::<_, _, String>("ALL_CLIENTS","$").unwrap();
    let client_list = serde_json::from_str::<Vec<Vec<CacheClient>>>(&client_list_string).unwrap();
    let client_list =client_list.get(0).unwrap();

    match client_list.iter().find(|client| client.id == user_id) {
        Some(client) => Some(client.clone()),
        None => None
    }
}

pub fn cache_client_in_game(redis_connection: &mut redis::Connection, user_id: uuid::Uuid, game_id: String) -> CacheClient {
    let client_list_string = redis_connection.json_get::<_, _, String>("ALL_CLIENTS", "$").unwrap();
    let client_list = serde_json::from_str::<Vec<Vec<CacheClient>>>(&client_list_string).unwrap();
    let client_list =client_list.get(0).unwrap();

    let client_index = client_list.iter().position(|client| client.id == user_id).unwrap();
    let mut client = client_list.get(client_index).unwrap().clone();
    client.state = ClientState::InGame(game_id);

    redis_connection.json_set::<_, _, CacheClient, ()>("ALL_CLIENTS",format!("$[{}]", client_index),&client ).unwrap();
    
    client
}

pub fn cache_client_online(redis_connection: &mut redis::Connection, user_id: uuid::Uuid) -> CacheClient {
    let client_list_string = redis_connection.json_get::<_, _, String>("ALL_CLIENTS", "$").unwrap();
    let client_list = serde_json::from_str::<Vec<Vec<CacheClient>>>(&client_list_string).unwrap();
    let client_list =client_list.get(0).unwrap();
    let client_index = client_list.iter().position(|client| client.id == user_id);
    let client_index = match client_index {
        Some(index) => index,
        None => return CacheClient {
            id: uuid::Uuid::nil(),
            username: "".to_string(),
            rank: "".to_string(),
            state: ClientState::Disconnected
        }
    };
    let mut client = client_list.get(client_index).unwrap().clone();
    client.state = ClientState::Online;

    redis_connection.json_set::<_, _, CacheClient, ()>("ALL_CLIENTS",format!("$[{}]", client_index),&client ).unwrap();
    
    client
}

pub fn uncache_client(mut redis_connection: redis::Connection, user_id: uuid::Uuid) -> CacheClient {
    let client_list_string = redis_connection.json_get::<_, _, String>("ALL_CLIENTS", "$").unwrap();
    let client_list = serde_json::from_str::<Vec<Vec<CacheClient>>>(&client_list_string).unwrap();
    let client_list =client_list.get(0).unwrap();
    let client_index = client_list.iter().position(|client| client.id == user_id);
    let client_index = match client_index {
        Some(index) => index,
        None => return CacheClient {
            id: uuid::Uuid::nil(),
            username: "".to_string(),
            rank: "".to_string(),
            state: ClientState::Disconnected
        }
    };
    let mut client = client_list.get(client_index).unwrap().clone();

    let mut cmd = cmd("JSON.ARRPOP");
    cmd.arg("ALL_CLIENTS").arg("$").arg(client_index as i64);

    let _: () = cmd.query(&mut redis_connection).unwrap();
    client.state = ClientState::Disconnected;
    client
}