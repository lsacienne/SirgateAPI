use diesel::{ExpressionMethods, prelude::*};
use crate::models::client::Client;
use crate::models::ranking::Rank;

pub fn get_all_ranks(connection: &mut PgConnection) -> Vec<Rank> {
    use crate::schema::rank::dsl::*;

    rank
        .load::<Rank>(connection)
        .expect("Error loading ranks")
}

pub fn get_users_by_rank(connection: &mut PgConnection, _rank_name: &str) -> Vec<Client> {
    use crate::schema::rank::dsl::*;

    let query_rank = rank
        .filter(rank_name.eq(_rank_name))
        .first::<Rank>(connection)
        .expect("Error loading rank");

    Client::belonging_to(&query_rank)
        .load::<Client>(connection)
        .expect("Error loading users")
}

pub fn get_users_grouped_by_ranks(connection: &mut PgConnection) -> Vec<(Rank, Vec<Client>)> {
    use crate::schema::rank::dsl::*;

    rank
        .load::<Rank>(connection)
        .expect("Error loading ranks")
        .into_iter()
        .map(|r| (r.clone(), get_users_by_rank(connection, &r.rank_name)))
        .collect()
}

pub fn update_rank(connection: &mut PgConnection, client_id: uuid::Uuid, _rank_name: &str) -> Client {
    use crate::schema::rank::dsl::*;
    use crate::schema::client::dsl::*;

    let query_rank = rank
        .filter(rank_name.eq(_rank_name))
        .first::<Rank>(connection)
        .expect("Error loading rank");

    diesel::update(Client::belonging_to(&query_rank))
        .filter(crate::schema::client::id.eq(client_id))
        .set(rank_id.eq(query_rank.id))
        .get_result(connection)
        .expect("Error updating rank")
}

pub fn update_rank_by_name(connection: &mut PgConnection, client_name: &str, _rank_name: &str) -> Client {
    let client_id = crate::controller::client::get_user_by_username(connection, client_name).id;
    update_rank(connection, client_id, _rank_name)
}