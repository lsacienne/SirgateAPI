use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::DbPool;

#[actix_web::post("/achievement/add")]
pub async fn add_achievement(pool: web::Data<DbPool>, achievement: web::Json<String>) -> impl Responder {
    // Deserialize JSON to Achievement struct
    let Achievement = achievement.into_inner();

    // TODO add achievement to db
    // Get UUID from db

    HttpResponse::Ok().body("Achievement Added")
}

#[actix_web::get("/achievement/getall")]
pub async fn get_all_achievements(pool: web::Data<DbPool>, client: web::Json<String>) -> actix_web::Result<impl Responder> {
    // Here you can add the user to the database.
    let client_id_string = client.into_inner();
    let id = Uuid::parse_str(&client_id_string).unwrap();

    let achievements = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::controller::achievement::get_all_achievements(&mut conn, id)
    }).await?;

    Ok(HttpResponse::Ok().json(achievements))
}
