use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use crate::models::achievement::{Achievement, ClientAchievement};

pub fn is_achievement_possessed(
    connection: &mut PgConnection,
    _client_id: uuid::Uuid,
    achievement_name: &str
) -> Result<Achievement, diesel::result::Error> {
    use crate::schema::client_achievement;
    use crate::schema::achievement;

    client_achievement::table
        .inner_join(achievement::table.on(achievement::id.eq(client_achievement::achievement_id)))
        .filter(
            client_achievement::client_id.eq(_client_id).and(achievement::name.eq(achievement_name))
        )
        .select(achievement::all_columns) // Select the desired columns
        .first(connection)
}

pub fn add_achievement(
    connection: &mut PgConnection,
    client_id: uuid::Uuid,
    achievement_name: &str
) -> Achievement {
    use crate::schema::achievement::dsl::*;
    use crate::schema::client_achievement;

    let test_achievement = is_achievement_possessed(connection, client_id, achievement_name);

    match test_achievement {
        Ok(_achievement) => _achievement,
        Err(_) => {
            let inserted_achievement = achievement
                .filter(name.eq(achievement_name))
                .first::<Achievement>(connection)
                .expect("Error loading achievement");
            let new_client_achievement = ClientAchievement {
                client_id,
                achievement_id: inserted_achievement.id
            };
        
            diesel::insert_into(client_achievement::table)
                .values(&new_client_achievement)
                .execute(connection)
                .expect("Error saving new achievement");
            inserted_achievement
        }
    }
}