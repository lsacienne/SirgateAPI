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