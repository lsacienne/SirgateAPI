use diesel::{associations::Identifiable, deserialize::Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::friend)]
#[diesel(primary_key(client1_id, client2_id))]
pub struct Friend {
    pub client1_id: uuid::Uuid,
    pub client2_id: uuid::Uuid
}

#[derive(Serialize, Deserialize)]
pub struct FriendAuth {
    pub client1_id: String,
    pub client2_id: String
}