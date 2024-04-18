use diesel::{associations::{Identifiable, Associations}, deserialize::Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use crate::models::ranking::Rank;

#[derive(Queryable, Selectable, Serialize, Associations, Identifiable, Deserialize)]
#[diesel(table_name = crate::schema::client)]
#[diesel(belongs_to(Rank))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub role_id: i32,
    pub rank_id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientAuth {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::client)]
pub struct InsertableClient<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub salt: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DGS{
    pub id: i32,
    pub label: String
}

