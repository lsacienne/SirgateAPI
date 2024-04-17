use diesel::{associations::Identifiable, deserialize::Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::rank)]
pub struct Rank {
    pub id: i32,
    pub rank_name: String,
}
