use diesel::{associations::{Identifiable, Associations}, deserialize::Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};
use crate::models::client::Client;

#[derive(Queryable, Selectable, Serialize, Identifiable, Deserialize)]
#[diesel(table_name = crate::schema::achievement)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Achievement {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub image: Option<String>
}

#[derive(Queryable, Insertable, Selectable, Associations, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::client_achievement)]
#[diesel(belongs_to(Client))]
#[diesel(belongs_to(Achievement))]
#[diesel(primary_key(client_id, achievement_id))]
pub struct ClientAchievement {
    pub client_id: uuid::Uuid,
    pub achievement_id: uuid::Uuid
}
